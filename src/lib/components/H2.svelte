<script lang="ts">
	export let id: string = '';
</script>

<h2 {id} class="text-xl font-semibold mb-3 mt-4">
	<slot />
</h2>
