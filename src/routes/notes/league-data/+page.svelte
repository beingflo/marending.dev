<script lang="ts">
	import Note from '$lib/components/Note.svelte';
	import Title from '$lib/components/Title.svelte';
	import { getData } from './utils';
	import MatchesHeatmap from './MatchesHeatmap.svelte';
	import MatchesHeatmapFma from './MatchesHeatmapFMA.svelte';
	import MatchesPerPlayer from './MatchesPerPlayer.svelte';
	import MatchesBar from './MatchesBar.svelte';
	import GoalsViaMatches from './GoalsViaMatches.svelte';

	const matches = getData();
</script>

<Note>
	<Title subtitle="Nobody tell management">9 years worth of company foosball data analyzed</Title>
	<MatchesHeatmap {matches} />
	<MatchesBar {matches} />
	<MatchesHeatmapFma {matches} />
	<MatchesPerPlayer {matches} />
	<GoalsViaMatches {matches} />
</Note>
