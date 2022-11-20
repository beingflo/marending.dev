<script lang="ts">
	export let title: string;
</script>

<details class="p-2 mb-3 border border-black bg-gray-50">
	<summary>
		{title}
	</summary>
	<div class="pt-2">
		<slot />
	</div>
</details>
