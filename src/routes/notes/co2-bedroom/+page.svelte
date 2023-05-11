<script>
	import Note from '$lib/components/Note.svelte';
	import Title from '$lib/components/Title.svelte';
	import Chart from '$lib/components/Chart.svelte';
	import * as Plot from '@observablehq/plot';
	import { dataClosedDoor, dataOpenDoor } from './data';
	import P from '$lib/components/P.svelte';
	import A from '$lib/components/A.svelte';
	import Hint from '$lib/components/Hint.svelte';
	import Info from '$lib/components/Info.svelte';
</script>

<Note>
	<Title subtitle="Trying not to suffocate while sleeping"
		>The air quality impact of closing your bedroom door at night</Title>
	<P
		>My recent foray into embedded devices has yielded a nice little CO2 sensor based on the <A
			href="http://www.seeedstudio.com/Grove-CO2-Temperature-Humidity-Sensor-SCD30-p-2911.html"
			>Seeed Grove SCD30</A
		>. I normally place it in the living room and ingest all the data into my Grafana / Prometheus
		based monitoring system. Unfortunately looking at some pretty graphs that tell you that you
		should probably open some windows gets a bit old at times. So I decided to conduct some
		experiments with the sensor instead.</P>
	<Info>What follows is highly unscientific (n=1). Proceed with caution.</Info>
	<P
		>Most people living with roommates probably have the habit of closing the bedroom door at night.
		It's also not uncommon to have someone over at night. The same applied for me, only when we
		moved in together in a new apartment did we start to leave the bedroom door open at night.</P>
	<P
		>The question arises immediately what happens to the CO2 level in each scenario. First, I placed
		the sensor in the bedroom and left the door open:</P>
	<div class="my-4">
		<Chart
			options={{
				y: { label: '↑ co2 (ppm)', domain: [0, 1050] },
				x: { type: 'time' },
				style: { fontSize: '12px' },
				marginLeft: 60,
				marks: [
					Plot.lineY(
						dataOpenDoor?.map(({ date, value }) => ({
							date: new Date(date),
							value,
						})),
						{ x: 'date', y: 'value' }
					),
					Plot.ruleY([1000], { stroke: 'orange', strokeDasharray: 2 }),
				],
			}} />
	</div>
	<P
		>As the night progresses we can observe a slight rise in CO2 concentration, but the large body
		of air in the entire apartment makes sure that the air quality stays in a good range.</P>
	<P
		>Next, I repeated the experiment with the bedroom door closed. Two people, one CO2 sensor, very
		romantic:</P>
	<div class="my-4">
		<Chart
			options={{
				y: { label: '↑ co2 (ppm)' },
				x: { type: 'time' },
				marginLeft: 60,
				style: { fontSize: '12px' },
				marks: [
					Plot.lineY(
						dataClosedDoor?.map(({ date, value }) => ({
							date: new Date(date),
							value,
						})),
						{ x: 'date', y: 'value' }
					),
					Plot.ruleY([1000], { stroke: 'orange', strokeDasharray: 2 }),
					Plot.ruleY([2000], { stroke: 'red', strokeDasharray: 2 }),
				],
			}} />
	</div>
	<P
		>I was shocked to see just how high the CO2 concentration goes in a relatively short period of
		time. Interesting is also the sharp decline once we got up and opened the door.
	</P>
	<P
		>I'm obviously not qualified to make claims about health impacts, but I wouldn't be surprised if
		this CO2 level affects sleep quality. For reference, some <A
			href="https://www.dhs.wisconsin.gov/chemical/carbondioxide.htm"
			>information on the internet</A> suggests that a conentration starting at 2000 ppm is associated
		with increased heart rate and headaches.</P>
	<P>Needless to say we will leave our door open.</P>
</Note>
