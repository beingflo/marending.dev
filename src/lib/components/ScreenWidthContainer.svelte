<div class="unset-margin"><slot /></div>

<style>
	.unset-margin {
		margin-left: min(calc((0rem - 100vw + 56rem) / 2), 0rem);
		width: calc(100vw - 2rem);
	}
</style>
