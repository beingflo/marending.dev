<script>
	import C from '$lib/components/C.svelte';
	import Code from '$lib/components/Code.svelte';
	import H2 from '$lib/components/H2.svelte';
	import H3 from '$lib/components/H3.svelte';
	import Info from '$lib/components/Info.svelte';
	import Note from '$lib/components/Note.svelte';
	import P from '$lib/components/P.svelte';
	import Title from '$lib/components/Title.svelte';
</script>

<Note>
	<Title subtitle="There are many like it, but this one is mine">About this website</Title>
	<P>
		Some time ago I decided to build a personal website to write about technical things I'm up to.
		If you're reading this, suprisingly, I followed through. Now, I'm under no delusion that the
		things I have to share will not be world shattering. Instead, I primarily want to write about
		projects to motivate myself, force a more rigorous thought process and document stuff for future
		reference. If the occasional internet user who happens to find their way to my little corner of
		the internet learns something, <em>anything</em>, that's just the cherry on top.
	</P>
	<P>
		I'm generally bad at putting stuff out there due to a case of perfectionism induced
		procrastination. So to counteract this tendency, I'm trying to write notes and publish projects
		even if they are not finished yet. In this spirit, you could call this space a
		<em>digital garden</em>.
	</P>
	<P
		>As such, notes on this website should never be considered finished. I will amend them if I see
		fit, rephrase if necessary or even delete them if proven not to stand the test of time. They are
		also published without to much regard for proof-reading and editing. If you find a typo or other
		issue, I'd love to hear about it via the feedback box on the bottom right of every page.
	</P>
	<H2>Technical setup</H2>
	<P
		>When contemplating the architecture of this site, the obvious (and probably sensible) choice
		was to use a static site generator. Static site generators take text files containing notes,
		typically in markdown format, and generate a static site that can easily be deployed. This
		approach has many advantages and a lot of tooling ready to go. However, it also limits the
		freedom to create custom design or functionality.
	</P>
	<P
		>Instead, I went with a custom application built with SvelteKit. Every page on this site is
		built from the ground up and can thus be customized completely. To have some consistency , I
		hope to build up a collection of reusable components that I can put to use in any note. Take for
		instance the component I wrap every note with:
	</P>
	<Code
		value={`<article class="p-4 py-8 md:py-12 max-w-4xl mx-auto">
	<slot />
</article>`} />
	<P
		>These three lines define the markup and styling of a container with responsive padding and
		max-width behaviour. The following snippet shows the code that generates the above snippet (hold
		on to your seat, we're going meta):
	</P>
	<Code
		value={`<Code value={\`<article class="p-4 py-8 md:py-12 max-w-4xl mx-auto"><slot /></article>\`} />`} />
	<P>
		Inside the <C>Code</C> component is some complexity related to code highligthing that would be unnecessary
		to unpack here.
	</P>
	<H3>Deployment</H3>
	<P>
		The SvelteKit application is built with the static adapter. This outputs only static assets that
		I'm serving directly from my webserver. I use caddy as the webserver as well as a reverse proxy
		to route the requests to the other applications I have running.
	</P>
</Note>
