<script lang="ts">
  export let message = '';
</script>

{#if message}
  <div class="reaction">{message}</div>
{/if}

<style>
  .reaction {
    display: inline-flex;
    align-items: center;
    min-height: 34px;
    padding: 0 12px;
    border-radius: 999px;
    background: rgba(245, 240, 228, 0.86);
    color: var(--ink);
    border: 1px solid rgba(116, 92, 46, 0.16);
    box-shadow: 0 8px 20px rgba(23, 22, 19, 0.16);
    font-size: 0.84rem;
    white-space: nowrap;
  }
</style>
