<script>
	import P from '$lib/components/P.svelte';
	import Note from '$lib/components/Note.svelte';
	import Title from '$lib/components/Title.svelte';
	import A from '$lib/components/A.svelte';
	import Details from '$lib/components/Details.svelte';
	import C from '$lib/components/C.svelte';
</script>

<Note>
	<Title date="03 December 2022" subtitle="Some schmuck tries his hands at generative art">
		A study of lines
	</Title>
	<P>
		After moving into a bigger apartement, I've found the walls of our office to be a bit naked.
		While I'm a hobby photographer and would like to get more into printing my work, for this space
		I have something else in mind.
	</P>
	<P>
		It started with finding out about <em>generative art</em> via
		<A href="https://tylerxhobbs.com/fidenza">Tyler Hobbs' Fidenza</A>. I found the look of this
		type of art very appealing. Maybe because I lack creativity, I like the idea of <em>guiding</em>
		randomness towards aestethic order, rather than <em>create</em> from scratch.
	</P>
	<P>
		On this page I want to systematically explore techniques and patterns to generate interesting
		structures. To prevent myself from getting lost in the endless possibilities, I will restrict
		myself to <span class="hover:line-through decoration-1 font-bold">lines</span>.
	</P>
	<Details title="How do I go about generating my lines?">
		Initially I had two programming languages in mind: <C>Rust</C> or <C>Typescript</C>. <C>Rust</C>
		being my favorite language was an obvious choice. However, the dynamic nature of <C>TS</C> lends
		itself for a problem like this. No sense in fighting <C>Rusts</C> borrow-checker for interactive
		code like this, and the upsides you get in turn don't really come into play here. What further seals
		the deal for me is that <C>TS</C>-based code can be natively embedded right here, on this page.
		<br />
		Next, I was exploring libraries that would make the whole ordeal easier. Top contenders include
		<A href="https://p5js.org/">p5js</A>, <A href="https://ptsjs.org/">Pts.js</A> and <A
			href="http://paperjs.org/">Paper.js</A
		>. Ultimately, I came to the conclusion that I don't need the extensive feature sets of any of
		these libraries. Animation, physics and geometry utilities are nice-to-haves but unlikely to see
		any use in my use-case. Instead, all I really need is a way to programatically draw lines,
		ideally in a way that I can get a high-resolution image out of it somehow for print. And what
		better way to do this than with our good friend, the <C>SVG</C>. To get a slightly nicer
		interface I use
		<A href="https://svgjs.dev/docs/3.0/">SVG.js</A>.
	</Details>
</Note>
