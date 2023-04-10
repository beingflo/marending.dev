<script>
	import A from '$lib/components/A.svelte';
	import C from '$lib/components/C.svelte';
	import Code from '$lib/components/Code.svelte';
	import Details from '$lib/components/Details.svelte';
	import H2 from '$lib/components/H2.svelte';
	import Hint from '$lib/components/Hint.svelte';
	import Info from '$lib/components/Info.svelte';
	import Note from '$lib/components/Note.svelte';
	import P from '$lib/components/P.svelte';
	import Title from '$lib/components/Title.svelte';
	import ResultsEspNow from './results-esp-now.svelte';
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
}`} />
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
		title="If you're like me you know nothing about ESP-NOW, so let's explore it a bit first">
		<P>
			ESP-NOW is designed for small messages up to 250 bytes. It can send messages to specific
			receivers or broadcast to anyone in listening range, without the need to establish a
			connection first. This is bound to give us an advantage when we try to minimize the time to
			ship data off device.
		</P>
		<P
			>The first thing I was interested to know after getting a proof of concept to work was how
			high the message throughput is. The very first test when sending from the XIAO to the
			FireBeetle showed just 3 messages per second. Very disappointing! Curiously, when the device
			is connected to a serial monitor, it's <em>much</em> faster. It turns out that there is a <A
				href="https://github.com/espressif/arduino-esp32/issues/6983">bug in Arduino</A> regarding buffering
			serial output. The fix is simple:
		</P>
		<Code value={`Serial.setTxTimeoutMs(0);`} respectMargin />
		<P>With this, I see around <b>800 messages per second</b> at a message size of 16 bytes.</P>
		<P
			>Another gotcha I encountered is how sensitive the antennas are to interference when the
			signal gets weak. For instance, when the sender or receiver is in my laundry room (1 floor
			down and a couple walls in between) the devices need to be kept well away of any metal object,
			otherwise no transmission will succeed. Even a metal baseplate that attaches legs to my dining
			room table are enough to block the signal from a device laying on the table. Ask me how I
			know.
		</P>
		<P
			>In circumstances with weak signal strength it is also noticable how sometimes a couple of
			retries are required. I would theorize this is due to WiFi interference on the 2.4 GHz band.</P>
	</Details>
	<P
		>Let's get to testing ESP-NOW. The following script starts up, sets everything up to be able to
		transmit data via ESP-NOW and then attempts to resend data until the recipient (in this case the
		device with the mac address described in the <C>receiver[]</C> array) acknowledges it. Then the device
		enters deep sleep for a microsecond, wakes up and reruns the procedure.</P>
	<Code
		caption="Code 3: Test script to measure time from deep sleep to successful data transmission with ESP-NOW"
		value={`#include <esp_now.h>
#include <WiFi.h>
#include <esp_wifi.h>

uint8_t receiver[] = {0x24, 0x4C, 0xAB, 0x82, 0xF5, 0x0C};

esp_now_peer_info_t peerInfo;

RTC_DATA_ATTR int counter = 0;
int success = 0;

struct msg_data
{
	int counter;
	int num_tries;
};

void OnDataSent(const uint8_t *mac_addr, esp_now_send_status_t status)
{
	Serial.println(status == ESP_NOW_SEND_SUCCESS ? "Success" : "Fail");

	if (status == ESP_NOW_SEND_SUCCESS)
	{
		success = 1;
	}
}

void setup()
{
	Serial.begin(115200);
	// For ESP32-C3
	// Serial.setTxTimeoutMs(0);
	WiFi.mode(WIFI_STA);

	if (esp_now_init() != ESP_OK)
	{
		Serial.println("Error initializing ESP-NOW");
		return;
	}

	if (esp_now_register_send_cb(OnDataSent) != ESP_OK)
	{
		Serial.println("Error registering callback");
		return;
	}

	memcpy(peerInfo.peer_addr, receiver, 6);
	peerInfo.channel = 0;
	peerInfo.encrypt = false;

	if (esp_now_add_peer(&peerInfo) != ESP_OK)
	{
		Serial.println("Error adding peer");
		return;
	}
}

void loop()
{
	esp_wifi_start();
	WiFi.mode(WIFI_STA);

	int num_tries = 0;

	// Resend until data was received by recipient
	while (!success)
	{
		esp_err_t result = ESP_FAIL;

		// Try sending until successfully sent
		while (result != ESP_OK)
		{
			num_tries += 1;

			struct msg_data data = {counter, num_tries};

			result = esp_now_send(receiver, (uint8_t *)&data, sizeof(msg_data));

			if (result != ESP_OK)
			{
				Serial.println("Error sending the data");
				break;
			}
		}

		// Allow time for callback to signal success
		delay(100);
	}

	success = 0;
	counter += 1;

	esp_wifi_stop();
	esp_sleep_enable_timer_wakeup(1);

	esp_deep_sleep_start();
	// or
	// esp_light_sleep_start();
}`} />
	<P>The results are very promising:</P>
	<ResultsEspNow />
	<P
		>Now we have some end-to-end timings to gauge how much time we should expect for a full cycle of
		waking up, sending data and resuming sleep in this protocol. Remember that assumption above,
		that the XIAO probably takes the same amount of time to wake from sleep? Clearly that didn't pan
		out if we consider the numbers here.</P>
	<P>Instead, the XIAO, or rather the ESP32-C3, is remarkably fast to wake from deep sleep.</P>
	<P
		>It will be interesting to consider in a next post which scenario is a particularly good fit for
		each of these boards. The FireBeetle has particularly good deep sleep current, but is slower to
		wake up and send data, whereas the XIAO has 4 times higher deep sleep current but is
		significantly faster to wake up and send data. Naturally, the XIAO should do better the higher
		the required frequency of measurments.</P>
	<Details title="Beware some pitfalls of light and deep sleep">
		<P>
			The loop is useless for the deep sleep test as upon wake up, the device starts executing from
			the start, rerunning the setup. This is unintuitive if one expects the code to resume from the
			invocation of <C>esp_deep_sleep_start()</C>. However, it makes sense once we consider that in
			this sleep mode just about everything is turned off to save power, including the memory that
			stores program state. That's why you'll notice the counter to be stored in RTC memory (via <C
				>RTC_DATA_ATTR</C
			>), this makes sure it survives the deep sleep.
		</P>
		<P
			>It took me some time to get this test to work with light sleep. The problem here is that
			while the program state is persisted and the code resumes as you would expect, the modem is
			powered down to save power. To be able to start it up properly, we <em>need</em> to call <C
				>esp_wifi_stop()</C> prior to starting sleep, otherwise you will not be able to send ESP-NOW
			messages.
		</P>
	</Details>
	<Hint>Under construction ...</Hint>
</Note>
