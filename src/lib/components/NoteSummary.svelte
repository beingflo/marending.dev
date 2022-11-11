<script lang="ts">
	export let title: string;
	export let summary: string;
	export let date: string;
	export let link: string;
</script>

<a class="w-fit" href={link}>
	<p class="text-sm md:text-md text-gray-600">{date}</p>
	<h3 class="text-md md:text-lg font-bold">{title}</h3>
	<p class="text-sm md:text-md">{summary}</p>
</a>
