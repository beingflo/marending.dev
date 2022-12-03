<script lang="ts">
	export let href: string;
</script>

<a {href} class="underline decoration-1 border-dashed">
	<slot />
</a>
