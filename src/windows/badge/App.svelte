<script lang="ts">
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { onDestroy, onMount } from 'svelte';

  let sessions: { id: string; state: string; agent: string; summary?: string }[] = [];
  let unlisten: UnlistenFn | null = null;

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

  onMount(async () => {
    unlisten = await listen<{ sessions: typeof sessions }>('badge-panel-data', ({ payload }) => {
      sessions = payload.sessions;
    });
  });

  onDestroy(() => {
    unlisten?.();
  });
</script>

<div class="panel">
  <div class="title">Sessions ({sessions.length})</div>
  {#if sessions.length === 0}
    <div class="empty">No active sessions</div>
  {:else}
    {#each sessions as session (session.id)}
      <div class="row">
        <div class="dot" style:background-color={badgeColor(session.state)}></div>
        <div class="info">
          <span class="summary">{session.summary || session.agent}</span>
          <span class="state">{session.state}</span>
        </div>
      </div>
    {/each}
  {/if}
</div>

<style>
  .panel {
    background: rgba(18, 20, 28, 0.95);
    border: 1px solid rgba(216, 165, 108, 0.2);
    border-radius: 10px;
    padding: 12px 14px;
    color: #f5f1e8;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
    backdrop-filter: blur(12px);
    width: 100%;
    height: 100%;
    overflow-y: auto;
  }
  .title {
    font-size: 10px;
    font-weight: 700;
    color: #bdb3a3;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    margin-bottom: 10px;
  }
  .empty {
    font-size: 11px;
    color: #9b917f;
  }
  .row {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 6px 0;
  }
  .row + .row {
    border-top: 1px solid rgba(255, 255, 255, 0.06);
  }
  .dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    margin-top: 3px;
    flex-shrink: 0;
  }
  .info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }
  .summary {
    font-size: 12px;
    font-weight: 500;
    color: #e8dccd;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .state {
    font-size: 10px;
    color: #9b917f;
  }
</style>
