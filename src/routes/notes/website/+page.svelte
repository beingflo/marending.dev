<script>
	import Code from '$lib/components/Code.svelte';
	import Details from '$lib/components/Details.svelte';
	import H2 from '$lib/components/H2.svelte';
	import H3 from '$lib/components/H3.svelte';
	import Hint from '$lib/components/Hint.svelte';
	import Note from '$lib/components/Note.svelte';
	import P from '$lib/components/P.svelte';
	import Title from '$lib/components/Title.svelte';
</script>

<Note>
	<Title date="27 November 2022" subtitle="There are many like it, but this one is mine"
		>About this website</Title>
	<P>
		Some time ago I decided to build a personal website to write about technicals things I'm up to.
		If you're reading this, suprisingly, I probably followed through. Now, I'm under no delusion
		that the things I have to share will not be mind blowing to many people out there. Instead, I
		primarily want to write about projects to motivate myself, force a more rigorous
		(thought-)process and document stuff for future reference. If one or the other reader who
		happens to stumble about my little corner of the internet learns something, <em>anything</em>,
		that's just the cherry on top.
	</P>
	<P>
		I'm generally bad at putting stuff out there due to a case of perfectionism induced
		procrastination. So to counteract this tendency, I'm trying to just write notes and publish
		projects even if they are not finished yet. In the context of personal blogs, I've stumbled
		across the term <em>digital garden</em>.
	</P>
	<Details title="What's a digital garden?">
		A website could be a perfectly polished space for content marketing of a company. Or, it could
		be a space for the organic growth of ideas. Just like a garden, that might be a clinical
		environment for industrial-scale food production, or a personal space that occasionally
		overgrows with weeds. <br />
		To consider my website a digital garden makes it explicit that it's a perpetual work in progress.
		Things are growing here, and might not be ready to win any prices just yet.
	</Details>
	<P
		>As such, notes on this website should never be considered finished. I will amend them if I see
		fit, rephrase if necessary or even delete if proven not to stand the test of time. They are also
		published without to much regard for proof-reading and editing. If you find a typo or other
		issue, I'd love to hear about it via the feedback box on the bottom right of every page.
	</P>
	<H2>Techical setup</H2>
	<P
		>When contemplating the architecture of this site, the obvious (and probably sensible) choice
		was using a static site generator. Static site generators take text files containing notes,
		typically in markdown format, and generate a static site that can easily be deployed. This
		approach has many advantages and many generators ready to go. However, it also limits the
		freedom to create custom design or functionality.
	</P>
	<P
		>Instead, I went with a custom application built on sveltekit. Every page on this site is built
		from the ground up and can thus be customized completely. To have some consistency, I hope to
		build up a collection of reusable components that I can put to use in any note. Take for
		instance the component I wrap every note with:
	</P>
	<Code
		value={`<article class="p-4 py-8 md:py-12 max-w-4xl mx-auto">
	<slot />
</article>`} />
	<P
		>These three lines define the markup and styling of a container with responsive padding and
		max-width behaviour.</P>
	<H3>Deployment</H3>
	<P
		>The sveltekit application is built using the node adapter and baked into a docker container.
		Lastly, said container is deployed to my small hetzner server.
	</P>
	<Hint>This post will get amended as I think of more info to add.</Hint>
</Note>
