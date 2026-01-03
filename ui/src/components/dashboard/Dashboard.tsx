import { createResource, Show } from 'solid-js';
import * as Plot from '@observablehq/plot';
import 'src/global.css';

export const Dashboard = () => {
	const fetchData = async () => {
		const response = await fetch('https://push.events.marending.dev/api/dashboard');
		if (!response.ok) throw new Error('Failed to fetch');
		const data = await response.json();
		return data;
	};

	const [data] = createResource(fetchData);

	return (
		<Show when={!data.loading}>
			<div class="p-8">
				<div class="text-xl">CO2: {data()?.co2_latest?.data?.[0]?.avg_co2?.toFixed(0)} ppm</div>
				<div class="text-xl mb-4">
					Humidity Laundry Room: {data()?.hum_latest?.data?.[0]?.humidity?.toFixed(2)}%
				</div>
				<div>
					{Plot.plot({
						y: { label: '↑ co2 (ppm)', domain: [300, 1400] },
						x: { type: 'time' },
						marginLeft: 60,
						marks: [
							Plot.lineY(
								data()?.co2_values?.data?.map(({ co2_ppm, time }: any) => ({
									time: new Date(time),
									value: co2_ppm,
								})),
								{ x: 'time', y: 'value' }
							),
						],
					})}
				</div>
			</div>
		</Show>
	);
};
