<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { onDestroy, onMount } from 'svelte';

  interface InteractionState {
    position_locked: boolean;
    click_through: boolean;
  }

  interface HitRegion {
    x: number;
    y: number;
    width: number;
    height: number;
  }

  interface HitLayoutPayload {
    width: number;
    height: number;
    regions: HitRegion[];
    pointer_alpha?: number;
  }

  const DRAG_THRESHOLD_PHYSICAL_PX = 3;

  let isDragging = false;
  let pointerActive = false;
  let activePointerId: number | null = null;
  let startX = 0;
  let startY = 0;
  let clickCount = 0;
  let clickTimer: ReturnType<typeof setTimeout> | null = null;
  let snapSide: 'left' | 'right' | null = null;
  let positionLocked = false;
  let promptedLockedMenu = false;
  let regions: HitRegion[] = [];
  let pointerAlpha = 0;
  let unlistenSnap: UnlistenFn | null = null;
  let unlistenInteraction: UnlistenFn | null = null;
  let unlistenLayout: UnlistenFn | null = null;

  // Badge state
  let badgeSessions: { id: string; state: string; agent: string; summary?: string }[] = [];
  let showBadgePanel = false;
  let unlistenBadge: UnlistenFn | null = null;

  function badgeColor(state: string): string {
    switch (state) {
      case 'working': case 'thinking': return '#ef4444';
      case 'idle': case 'attention': return '#22c55e';
      case 'juggling': case 'conducting': return '#3b82f6';
      case 'sleeping': return '#6b7280';
      case 'error': return '#f97316';
      default: return '#a78bfa';
    }
  }

  function onBadgeClick(e: Event) {
    e.stopPropagation();
    showBadgePanel = !showBadgePanel;
  }

  function onPointerDown(e: PointerEvent) {
    if (e.button !== 0) return; // Only handle left click — right click goes to onContextMenu
    isDragging = false;
    pointerActive = true;
    activePointerId = e.pointerId;
    promptedLockedMenu = false;
    startX = e.screenX;
    startY = e.screenY;
    // Capture pointer so events keep flowing even if cursor leaves the hit window
    // (critical for mini mode where the hit window is very small).
    (e.target as Element)?.setPointerCapture(e.pointerId);
    if (positionLocked) return;

    invoke('drag_start', { x: startX, y: startY });

    clickCount++;
    if (clickTimer) clearTimeout(clickTimer);
    clickTimer = setTimeout(() => {
      const count = clickCount;
      clickCount = 0;
      if (!isDragging) {
        if (count === 2) invoke('hit_double_click');
        else if (count >= 4) invoke('hit_flail');
      }
    }, 300);
  }

  function dragDistanceThresholdLogicalPx() {
    return DRAG_THRESHOLD_PHYSICAL_PX / (window.devicePixelRatio || 1);
  }

  function onPointerMove(e: PointerEvent) {
    if (!pointerActive) return;
    if (activePointerId !== null && e.pointerId !== activePointerId) return;
    if (e.buttons === 0) return;
    if (positionLocked) {
      const dx = e.screenX - startX;
      const dy = e.screenY - startY;
      if (!promptedLockedMenu && Math.sqrt(dx * dx + dy * dy) >= dragDistanceThresholdLogicalPx()) {
        promptedLockedMenu = true;
        invoke('show_context_menu');
      }
      return;
    }
    // Mark as dragging after crossing the same physical-pixel threshold as Rust.
    if (!isDragging) {
      const dx = e.screenX - startX;
      const dy = e.screenY - startY;
      if (Math.sqrt(dx * dx + dy * dy) >= dragDistanceThresholdLogicalPx()) isDragging = true;
    }
    invoke('drag_move', { x: e.screenX, y: e.screenY });
  }

  function onPointerUp(e: PointerEvent) {
    if (!pointerActive) return;
    if (activePointerId !== null && e.pointerId !== activePointerId) return;
    (e.target as Element)?.releasePointerCapture(e.pointerId);
    pointerActive = false;
    activePointerId = null;
    snapSide = null;
    promptedLockedMenu = false;
    if (!positionLocked) {
      invoke('drag_end');
    }
  }

  function onPointerCancel(e: PointerEvent) {
    if (!pointerActive) return;
    (e.target as Element)?.releasePointerCapture(e.pointerId);
    pointerActive = false;
    activePointerId = null;
    snapSide = null;
    promptedLockedMenu = false;
    if (!positionLocked) {
      invoke('drag_end');
    }
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault();
      invoke('hit_double_click');
    }
  }

  function onContextMenu(e: MouseEvent) {
    e.preventDefault();
    invoke('show_context_menu');
  }

  onMount(() => {
    const setup = async () => {
      const interaction = await invoke<InteractionState>('get_interaction_state');
      positionLocked = interaction.position_locked ?? false;

      unlistenSnap = await listen<{ active: boolean; side: 'left' | 'right' | null }>('snap-preview', ({ payload }) => {
        snapSide = payload.active ? payload.side : null;
      });

      unlistenInteraction = await listen<InteractionState>('interaction-state-changed', ({ payload }) => {
        positionLocked = payload.position_locked ?? false;
      });

      unlistenLayout = await listen<HitLayoutPayload>('hit-layout-changed', ({ payload }) => {
        regions = payload.regions ?? [];
        pointerAlpha = payload.pointer_alpha ?? 0;
      });

      unlistenBadge = await listen<{ sessions: { id: string; state: string; agent: string; summary?: string }[] }>('sessions-badge', ({ payload }) => {
        badgeSessions = payload.sessions.filter((s: { id: string }) => !s.id.startsWith('claude-monitor-'));
        if (badgeSessions.length === 0) showBadgePanel = false;
      });

      const initialLayout = await invoke<HitLayoutPayload | null>('get_current_hit_layout');
      if (initialLayout) {
        regions = initialLayout.regions ?? [];
        pointerAlpha = initialLayout.pointer_alpha ?? 0;
      }

      window.addEventListener('pointermove', onPointerMove);
      window.addEventListener('pointerup', onPointerUp);
      window.addEventListener('pointercancel', onPointerCancel);
    };
    setup();
  });

  onDestroy(() => {
    window.removeEventListener('pointermove', onPointerMove);
    window.removeEventListener('pointerup', onPointerUp);
    window.removeEventListener('pointercancel', onPointerCancel);
    unlistenSnap?.();
    unlistenInteraction?.();
    unlistenLayout?.();
    unlistenBadge?.();
    if (clickTimer) clearTimeout(clickTimer);
  });
</script>

<div class="hit-root" style={`--pointer-alpha:${pointerAlpha}`}>
  {#each regions as region, index (index)}
    <div
      class="hit-zone"
      class:locked={positionLocked}
      class:snap-left={snapSide === 'left'}
      class:snap-right={snapSide === 'right'}
      style:left={`${region.x}px`}
      style:top={`${region.y}px`}
      style:width={`${region.width}px`}
      style:height={`${region.height}px`}
      onpointerdown={onPointerDown}
      oncontextmenu={onContextMenu}
      onkeydown={onKeyDown}
      role="button"
      tabindex="0"
      aria-label="Clyde desktop pet"
    ></div>
  {/each}
  {#if badgeSessions.length > 0}
    <div class="badge-area">
      {#each badgeSessions as session (session.id)}
        <div
          class="badge-dot"
          style:background-color={badgeColor(session.state)}
          onclick={onBadgeClick}
          role="button"
          tabindex="0"
          aria-label={`${session.agent}: ${session.state}`}
        ></div>
      {/each}
    </div>
    {#if showBadgePanel}
      <div class="badge-panel" onclick={(e) => e.stopPropagation()}>
        <div class="badge-panel-title">Sessions ({badgeSessions.length})</div>
        {#each badgeSessions as session (session.id)}
          <div class="badge-panel-row">
            <div class="badge-panel-dot" style:background-color={badgeColor(session.state)}></div>
            <div class="badge-panel-info">
              <span class="badge-panel-agent">{session.agent}</span>
              <span class="badge-panel-state">{session.state}</span>
              {#if session.summary}
                <span class="badge-panel-summary">{session.summary}</span>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    {/if}
  {/if}
</div>

<style>
  .hit-root {
    position: relative;
    width: 100%;
    height: 100%;
    background: transparent;
    pointer-events: none;
  }

  .hit-zone {
    position: absolute;
    background: rgba(0, 0, 0, var(--pointer-alpha, 0));
    cursor: grab;
    pointer-events: auto;
    touch-action: none;
    user-select: none;
    -webkit-user-select: none;
  }

  .hit-zone.locked {
    cursor: not-allowed;
  }

  .hit-zone::after {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: 16px;
    opacity: 0;
    transition: opacity 120ms ease, box-shadow 120ms ease, border-color 120ms ease;
    pointer-events: none;
    border: 2px solid transparent;
  }

  .hit-zone.snap-left::after,
  .hit-zone.snap-right::after {
    opacity: 1;
    box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.12), 0 10px 24px rgba(59, 130, 246, 0.2);
  }

  .hit-zone.snap-left::after {
    border-left-color: rgba(59, 130, 246, 0.9);
    background: linear-gradient(90deg, rgba(59, 130, 246, 0.22), transparent 42%);
  }

  .hit-zone.snap-right::after {
    border-right-color: rgba(59, 130, 246, 0.9);
    background: linear-gradient(270deg, rgba(59, 130, 246, 0.22), transparent 42%);
  }

  /* Badge dots */
  .badge-area {
    position: absolute;
    left: 0;
    top: 50%;
    transform: translateY(-50%);
    display: flex;
    flex-direction: column;
    gap: 5px;
    z-index: 20;
    pointer-events: auto;
    cursor: pointer;
    padding: 4px;
  }
  .badge-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    border: 1.5px solid rgba(0, 0, 0, 0.3);
    pointer-events: auto;
    cursor: pointer;
    animation: badge-pulse 2s ease-in-out infinite;
    transition: transform 0.15s ease;
  }
  .badge-dot:hover {
    transform: scale(1.3);
  }
  @keyframes badge-pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.6; }
  }

  /* Badge detail panel */
  .badge-panel {
    position: absolute;
    left: 20px;
    top: 50%;
    transform: translateY(-50%);
    background: rgba(18, 20, 28, 0.95);
    border: 1px solid rgba(216, 165, 108, 0.2);
    border-radius: 10px;
    padding: 10px 12px;
    min-width: 160px;
    max-width: 220px;
    z-index: 30;
    backdrop-filter: blur(12px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
    pointer-events: auto;
  }
  .badge-panel-title {
    font-size: 10px;
    font-weight: 700;
    color: #bdb3a3;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    margin-bottom: 8px;
  }
  .badge-panel-row {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 4px 0;
  }
  .badge-panel-row + .badge-panel-row {
    border-top: 1px solid rgba(255, 255, 255, 0.05);
  }
  .badge-panel-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    margin-top: 3px;
    flex-shrink: 0;
  }
  .badge-panel-info {
    display: flex;
    flex-direction: column;
    gap: 1px;
    min-width: 0;
  }
  .badge-panel-agent {
    font-size: 11px;
    font-weight: 600;
    color: #f0e6d7;
  }
  .badge-panel-state {
    font-size: 10px;
    color: #bdb3a3;
  }
  .badge-panel-summary {
    font-size: 10px;
    color: #9b917f;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
</style>
