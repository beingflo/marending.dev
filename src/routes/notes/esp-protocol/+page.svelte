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
	<P
		>To wirelessly transmit sensor readings to either a Wifi router or some custom base-station
		device, I can think of the following protocols: Wifi, Bluetooth low energy (BLE) and ESP-NOW.
		Wifi is great because it allows the devices to connect directly to my Wifi router, no middle-man
		required. On a 2.4 GHz network it also has considerable range and can get signal through
		multiple walls. BLE on the other hand could get away with much less power consumption (although
		from what I read this is not particularly impressive on ESP32, and I'm too lazy to test it
		myself), at the expense of shorter range and less wall penetration.
	</P>
	<P
		>For the sake of a rough estimation, we will assume that while awake the device consumes the
		amout of power we measured during Wifi setup. There is some overlap between the different boards
		at around 60 mA, so let's go with that.</P>
</Note>
