<script>
	import A from '$lib/components/A.svelte';
	import Code from '$lib/components/Code.svelte';
	import Details from '$lib/components/Details.svelte';
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
	<H2>Boards under test</H2>
	<P
		>We measured power draw of four different development boards. Now that we know their
		characteristics (and I'm lazy), we will proceed to only consider two boards from here on out:
		The <A href="https://www.dfrobot.com/product-1590.html">DFRobot FireBeetle ESP32</A> and the
		<A href="https://www.seeedstudio.com/Seeed-XIAO-ESP32C3-p-5431.html"
			>Seeed Studio XIAO ESP32C3</A
		>. They represent a nice selection of the boards I own. The FireBeetle is the fully featured
		ESP32 board, with an ARM based chip with two cores, plenty of GPIO pins and a built-in battery
		connector. The XIAO on the other hand is tiny, more affordable and ships with an external
		antenna presumably due to a lack of space for a PCB antenna. It features the ESP32C3, a RISV-V
		based single core CPU.
	</P>
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
	<H2>Boot time</H2>
	<P
		>First it would be interesting to time the boot time of a board in isolation. That way we will
		have a reference to know how much time we actually spend in a transmission and how much is just
		waking up.</P>
	<P>
		To measure this I simply log some serial output, go to deep sleep and immediately wake up again
		and repeat. This shows around 7.5 wakeups per second on the Firebeetle ESP32 board, that gives a
		boot time of <b>133 ms</b>. For wakeup after light sleep I'm getting around <b>1.4 ms</b>. I
		couldn't test this on a Seeed XIAO because the serial monitor doesn't automatically reconnect,
		which makes it hard to observe the output. I'm sure this could be fixed if one wasn't a complete
		novice. I would assume it's not much different considering it has a chip in the same family.
		Once we get to the protocols we'll see if this assumption makes sense.
	</P>
	<Code
		caption="Code 1: Script to test deep sleep wakeup time"
		value={`#include "Arduino.h"

RTC_DATA_ATTR int value = 0;

void setup()
{
	Serial.begin(115200);

	Serial.println(value);
	value += 1;

	esp_sleep_enable_timer_wakeup(1);

	esp_deep_sleep_start();
}

void loop()
{
	// This is not going to be called
}
	`} />
	<P
		>This is quite promising for deep sleep wakeup! If we can keep the Modem startup and
		transmission time on a reasonable scale we'll have pretty good battery life.</P>
	<Code value={`WiFi.mode(WIFI_STA);`} caption="Code 2: Start the modem in WiFi station mode" />
	<P
		>When we add the above snippet to the test script we measure a total cycle time of <b>312 ms</b
		>. Starting the modem (at least I suspect this doesn't just set the mode considering the time
		increase) makes the startup process quite a bit slower.</P>
	<P>But enough groundwork, let's get to trialing the first protocol.</P>
	<H2>ESP-NOW</H2>
	<Details
		title="If you're like me you know nothing about ESP-NOW, so let's explore it a bit first:"
		>Background</Details>
</Note>
