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
	import Info from '$lib/components/Info.svelte';
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
	<H2>Methodology</H2>
	<P
		>Thanks to my multimeter, measuring the power consumption of the devices is not too hard. I just
		needed a power source I can connect in series with the multimeter, so I can take the
		measurements. The battery pack I have has a JST-PH2 connector which I don't want to cut off to
		get access to the wires. So instead, I wired together a bootleg battery by connecting three AA
		batteries in series, giving me 4.5 Volts.</P>
	<P
		>The dev boards need a lot of power to initially boot up and start running code. Once they enter
		sleep, the current drops down. A tiny hiccup here was to realize that I can't set my multimeter
		to the microamp setting from the beginning, as apparently not enough current makes it through
		the device to start up properly. Instead, I need to set it to miliamps at first, this way it can
		boot up and afterwards I can turn the setting to microamps without the power getting
		interrupted.
	</P>
	<Info>
		Note that these are mostly out of the box values a novice like myself would realistally achieve.
		I'm sure one could optimize the power consumption further with advanced tricks.</Info>
</Note>
