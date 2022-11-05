<script>
  export let data;
  const {rows} = data;
</script>

{#each rows as row}
  <p>{row.number} {row.text}</p>
{/each}
