<script>
	import A from '$lib/components/A.svelte';
	import Note from '$lib/components/Note.svelte';
	import P from '$lib/components/P.svelte';
	import Title from '$lib/components/Title.svelte';
</script>

<Note>
	<Title subtitle="Trying to sleep as much as possible"
		>A comparison of wireless protocols for battery powered embedded devices</Title>
	<P>
		<A href="/notes/esp-power">We've established</A> that deep sleep on ESP32 boards gives us a runtime
		of ~4 up to ~15 years on a modest battery charge. This is plenty for my applications. So now we need
		to turn our attention towards the awake time of our sensor devices. This will mostly consist of collecting
		a measurment, sending it off to some system collecting the metrics and then going back to sleep.
	</P>
</Note>
