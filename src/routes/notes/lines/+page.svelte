<script>
	import P from '$lib/components/P.svelte';
	import Note from '$lib/components/Note.svelte';
	import Title from '$lib/components/Title.svelte';
	import A from '$lib/components/A.svelte';
	import Details from '$lib/components/Details.svelte';
	import C from '$lib/components/C.svelte';
	import Circle from '$lib/components/lines/Circle.svelte';
	import Hint from '$lib/components/Hint.svelte';
	import SimpleLine from '$lib/components/lines/SimpleLine.svelte';
	import Chaos from '$lib/components/lines/Chaos.svelte';
	import Info from '$lib/components/Info.svelte';
	import ChaosWrong from '$lib/components/lines/ChaosWrong.svelte';
	import Code from '$lib/components/Code.svelte';
	import ChaosAngles from '$lib/components/lines/ChaosAngles.svelte';
	import ChaosAngles90 from '$lib/components/lines/ChaosAngles90.svelte';
</script>

<Note>
	<Title date="03 December 2022" subtitle="Some schmuck tries his hands at generative art">
		A study of lines
	</Title>
	<Hint>This note is a work in progress!</Hint>
	<P>
		After moving into a bigger apartement, I've found the walls of our office to be a bit naked.
		While I'm a hobby photographer and would like to get more into printing my work, for this space
		I have something else in mind.
	</P>
	<P>
		It started with finding out about <em>generative art</em> via
		<A href="https://tylerxhobbs.com/fidenza">Tyler Hobbs' Fidenza</A>. I found the look of this
		type of art very appealing. Maybe because I lack artistic talent, I like the idea of
		<em>guiding</em>
		randomness towards aestethic order, rather than <em>create</em> from scratch.
	</P>
	<P>
		On this page I want to systematically explore techniques and patterns to generate interesting
		structures. To prevent myself from getting lost in the endless possibilities, I will restrict
		myself to <span class="hover:line-through decoration-1 font-bold">straight lines</span>.
	</P>
	<Details title="How do I go about generating my lines?">
		Initially I had two programming languages in mind: <C>Rust</C> or <C>Typescript</C>. <C>Rust</C>
		being my favorite language was an obvious choice. However, the dynamic nature of <C>TS</C> lends
		itself to a problem like this. No sense in fighting <C>Rusts</C> borrow-checker for interactive code,
		and the upsides you get in turn don't really come into play here. What further seals the deal for
		me is that <C>TS</C>-based code can be natively embedded right here, on this page.
		<br /><br />
		Next, I was exploring libraries that would make the whole ordeal easier. Top contenders include
		<A href="https://p5js.org/">p5js</A>, <A href="https://ptsjs.org/">Pts.js</A> and <A
			href="http://paperjs.org/">Paper.js</A
		>. Ultimately, I came to the conclusion that I don't need the extensive feature sets of any of
		these libraries. Animation, physics and geometry utilities are nice-to-haves but unlikely to see
		any use in my use-case. Instead, all I really need is a way to programatically draw lines,
		ideally in a way that I can get a high-resolution image out of it somehow for print. And what
		better way to do this than with our good old friend, the <C>SVG</C>. So I just directly
		construct <C>SVGs</C> in my <C>Svelte</C> components.
	</Details>
	<P>So without further ado, let's draw some lines!</P>
	<Circle />
	<P
		>Wait, that's not a straight line! I just wanted to make sure you're paying attention. <q
			>Many infinitesimally small straight lines can approximate a curve</q
		>, you might say. <q>That violates the spirit of the exercise, stop raining on my parade!</q>, I
		would retort.
	</P>
	<SimpleLine />
	<P
		>Much better, we're checking all the boxes. Well, the one. If I was a famous artist already, I
		would hang this on the wall and have critics say clever sounding things about it. Alas, I need
		to try a bit harder. Let's try to bring some repetition into the mix, and while we're at it,
		some <span class="hover:rotate-45 transition inline-block">randomness</span>.
	</P>
	<Info>
		Note that all the pieces on this page that contain randomness are uniquely generated for each
		page view. Consider yourself special!
	</Info>
	<Chaos />
	<P>Now we're getting somewhere! It's no masterpiece, but I don't hate it.</P>
	<Details title="Excursion into wrong transforms">
		<P>
			Before I arrived at the above piece, my makeshift <C>SVG</C> rendering blessed me with this abomination:
		</P>
		<ChaosWrong />
		<P>
			The problem here is the the origin of the rotational transform. You see, drawing a line that
			is not horizontal in an <C>SVG</C> could be accomplished by providing the start and endpoint (which
			do not have the same <C>y</C> coordinate) explicitly. When generating lots of randomly rotated
			lines of the same length, we have to do some trigonometry to figure out one of those points. And
			I for one don't have time to use my brain like that.
		</P>
		<P>
			Instead, I want to draw my line horizontally, and then rotate it using the <C>rotate</C> transform
			that <C>SVG</C> provides.
		</P>
		<Code
			respectMargin
			value={`<svg width="600" height="400" xmlns="http://www.w3.org/2000/svg">
	<line x1={x} y1={y} x2={x + 50} y2={y} transform="rotate(rand())" stroke="black" />
</svg>`} />
		<P>
			This way I don't have to do any math, just generate random numbers for the rotation. As it
			turns out, this doesn't work correctly as we have seen above. The lines are rotated about the
			<C>(0, 0)</C> origin instead of the center point of the line. To fix it, we either set the <C
				>transform-origin</C> property on the <C>SVG</C> correctly, or we get fancy:
		</P>
		<Code
			respectMargin
			value={`<svg width="600" height="400" xmlns="http://www.w3.org/2000/svg">
	<line x1="-25" y1="0" x2="25" y2="0" 
		transform="rotate(rand()), translate(rand(), rand())" stroke="black" />
</svg>`} />
		<P>
			We draw lines with center points on the origin and use the <C>transform</C> both for rotation
			<em>and for translation</em>. This way we don't have to set the origin of transformation. Note
			the order of the transforms: They are applied from right to left. If we first translated, we
			would have the wrong transform origin again.
		</P>
	</Details>
	<Info
		>Fun fact: At this point in writing this note, the <C>/notes/lines</C> page you're currently viewing
		can be server-side rendered approximately 80 times per second. Compare that to 300 times per second
		for the landing page. All that <C>SVG</C> generation already takes its toll on that server.
	</Info>
	<P>
		While the above piece starts to look interesting, we have a bad case of <em
			>untamed randomness</em
		>. We might get more visually appealing structures by restricting e.g. the angle of the lines.
	</P>
	<ChaosAngles />
	<P>
		At this point I'm not sure if there is an optical illusion with a all these
		specs-of-dust-looking things between the lines or my monitor is dirty. We will never know.
	</P>
	<ChaosAngles90 />
	<P
		>Adding more smaller lines and rotating the lines around by 90 degrees makes this look a bit
		like rain.</P>
</Note>
