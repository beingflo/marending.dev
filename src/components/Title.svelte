<script lang="ts">
  export let subtitle: string;
</script>

<h1
  class="text-4xl md:text-6xl md:mb-2 text-black dark:text-white font-extrabold text-left"
>
  <slot />
</h1>
<h2 class="text-lg md:text-xl mb-16 md:mb-24 text-left">{subtitle}</h2>

<svelte:head>
  <meta name="description" content={subtitle} />
</svelte:head>
