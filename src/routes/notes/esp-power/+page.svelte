<script>
	import A from '$lib/components/A.svelte';
	import C from '$lib/components/C.svelte';
	import Code from '$lib/components/Code.svelte';
	import H2 from '$lib/components/H2.svelte';
	import H3 from '$lib/components/H3.svelte';
	import Hint from '$lib/components/Hint.svelte';
	import Note from '$lib/components/Note.svelte';
	import P from '$lib/components/P.svelte';
	import Title from '$lib/components/Title.svelte';
	import { cpp } from 'svelte-highlight/languages';
	import Details from '$lib/components/Details.svelte';
</script>

<Note>
	<Title subtitle="First forays into embedded devices"
		>Measuring power consumption of an ESP32 development board</Title>
	<P
		>I like some nice greenery in the living room. I also like data. Naturally, I have to monitor
		the CO2, temperature and humidity in the room as well as the soil moisture and incident light on
		the plant.
	</P>
	<P
		>Fine, maybe I don't <em>have</em> to, but it's fun anyway. To make it happen without a huge mess
		of cables, I would like to set up a system where the sensors are battery operated and report their
		measurements into my monitoring setup (prometheus as of now).</P>
	<P
		>Until now, I've used Raspberry Pis and similar devices as low power home servers. They are
		great as fully fledged file or web servers. For this endeavor however, they are clearly too
		power hungry. I'm envisioning months if not years of battery life, not just days. So I started
		playing with the idea of venturing into embedded devices. Since I have no idea what I'm doing, I
		chose a well trodden path: ESP32.
	</P>
	<P>
		As usual when I pick up a new topic of interest, I buy all kinds of stuff I probably don't need.
		In this case I got a range of dev boards, dozens of sensors, breadboards, cables, a ridiculous
		number of resistors and even a multimeter. You might think it's stupid to just buy a ton of
		stuff hoping it will keep an interest going, but think again: It sometimes works.
	</P>
	<P
		>For starters, I want to understand what kind of power envelopes I'm dealing with on these
		devices. So I run a couple of dev boards in different states: Normal operation without Wifi,
		normal operation with Wifi, light sleep and deep sleep.
	</P>
</Note>
