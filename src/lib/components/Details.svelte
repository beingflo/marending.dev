<script lang="ts">
	export let title: string;
</script>

<details class="mb-3 border border-black bg-gray-50">
	<summary class="cursor-pointer p-2">
		{title}
	</summary>
	<div class="p-2">
		<slot />
	</div>
</details>
