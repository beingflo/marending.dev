<script lang="ts">
	import Note from '$lib/components/Note.svelte';
	import Title from '$lib/components/Title.svelte';
	import { getData } from './utils';
	import MatchesHeatmap from './MatchesHeatmap.svelte';

	const matches = getData();
</script>

<Note>
	<Title subtitle="Don't tell managment what we're up to"
		>9 years worth of foosball data analyzed</Title>
	<MatchesHeatmap {matches} />
</Note>
