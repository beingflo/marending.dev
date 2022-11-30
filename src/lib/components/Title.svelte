<script lang="ts">
	export let date: string;
	export let subtitle: string;
</script>

<p class="text-gray-600 mb-2">{date}</p>
<h1 class="text-2xl md:text-4xl mb-1 text-black"><slot /></h1>
<h2 class="text-lg md:text-xl mb-16 md:mb-24">{subtitle}</h2>

<style>
	h1 {
		font-family: 'LatoLatinWebBlack';
	}
</style>
