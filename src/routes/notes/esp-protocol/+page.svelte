<script>
	import A from '$lib/components/A.svelte';
	import H2 from '$lib/components/H2.svelte';
	import Info from '$lib/components/Info.svelte';
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
		>To wirelessly transmit sensor readings to either a WiFi router or some custom base-station
		device, I can think of the following protocols: WiFi, Bluetooth low energy (BLE) and ESP-NOW.
	</P>
	<P>
		WiFi is great because it allows the devices to connect directly to my WiFi router, no middle-man
		required. On a 2.4 GHz network it also has considerable range and can get signal through
		multiple walls. BLE on the other hand could get away with much less power consumption (although
		from what I read this is not particularly impressive on ESP32, and I'm too lazy to test it
		myself), at the expense of shorter range and less wall penetration.
	</P>
	<P>
		The dark horse in the lineup is ESP-NOW. It's a proprietary protocol by espressif with some
		desirable properties for my use-cases: It's connectionless, which hopefully will shorten the
		time until a packet is sent, it allows broadcasting messages and it runs on the normal WiFi band
		meaning we inherit the range capabilities.
	</P>
	<P
		>Both BLE and ESP-NOW would require a gateway device that receives messages and sends them off
		via WiFi to my monitoring setup in the cloud.</P>
	<H2>Awake time is essential</H2>
	<P
		>For the sake of a rough estimation, we will assume that while awake the device consumes the
		amout of power we measured during WiFi setup. There is some overlap between the different boards
		at around <b>60 mA</b>, so let's go with that.</P>
	<Info
		>Of course there is going to be a difference in power draw between WiFi and BLE, they are
		fundamentally different in hardware. But as a simplification I will just gloss over this.</Info>
	<P
		>Comparing this number to a deep sleep current of 10 - 40 uA it's easy to see that it's
		essential for battery life to spend as little time as possible awake. An additional second of
		active processing can shorten the possible sleep time by almost 2 hours!
	</P>
	<P
		>For this reason we are going to measure how quickly a device can wake up from light or deep
		sleep, establish a connection (if necessary), send a small message with sensor readings and go
		back to sleep. Since the boot process of the chip is hard to time exactly, I figure the most
		effective way to measure the full cycle is to sleep only very briefly in-between cycles and
		check how many messages are received over time. If e.g. 2 messages reach the monitoring setup
		per second, then one full cycle takes 500 miliseconds. Or if only 1 message every 7 seconds
		arrives, clearly the cycles takes 7 second.
	</P>
</Note>
