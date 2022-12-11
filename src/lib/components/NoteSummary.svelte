<script lang="ts">
	export let title: string;
	export let summary: string;
	export let date: string;
	export let link: string;
</script>

<a class="w-fit" href={link}>
	<p class="text-sm text-gray-600">{date}</p>
	<h3 class="font-semibold">{title}</h3>
	<p class="text-sm font-light">{summary}</p>
</a>
