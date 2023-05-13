import data from './matches.json';

export type MatchRaw = {
	gameId: string;
	matchIndex: string;
	players: Array<string>;
	teamA: Array<string>;
	teamB: Array<string>;
	winProbabilityTeamA: number;
	scorers: Array<string>;
	tags: Array<string>;
	startDate: string;
	endDate: string;
	table: string;
	teamAScore: string;
	teamBScore: string;
	winPointsTeamA: string;
	winPointsTeamB: string;
	maxGoals?: string;
	maxMatches?: string;
	positionSwappingAllowed?: boolean;
	version?: string;
	goals?: Goals;
};

export type Match = {
	players: Array<string>;
	teamA: Array<string>;
	teamB: Array<string>;
	startDate: Date;
	endDate: Date;
	teamAScore: string;
	teamBScore: string;
	goals?: Goals;
};

export type Goals = {
	scorer: Array<string>;
	time: Array<string>;
};

export const getData = (): Array<Match> => {
	return (data as [])?.map((d: MatchRaw) => ({
		startDate: new Date(d.startDate?.slice(0, -4)),
		endDate: new Date(d.endDate?.slice(0, -4)),
		players: d.players,
		teamA: d.teamA,
		teamB: d.teamB,
		teamAScore: d.teamAScore,
		teamBScore: d.teamBScore,
		goals: d.goals,
	}));
};
