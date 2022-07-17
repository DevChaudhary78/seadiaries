<script></script>

<section class="blog-container">
	<div class="blog">
		<h2>Title</h2>
		<p>Body of the blog</p>
		<div>
			<p>321</p>
			<p>12</p>
			<p>Dev Chaudhary</p>
			<p>8th May</p>
		</div>
	</div>
	<div class="blog">
		<h2>Title</h2>
		<p>Body of the blog</p>
		<div>
			<p>321</p>
			<p>12</p>
			<p>Dev Chaudhary</p>
			<p>8th May</p>
		</div>
	</div>
	<div class="blog">
		<h2>Title</h2>
		<p>Body of the blog</p>
		<div>
			<p>321</p>
			<p>12</p>
			<p>Dev Chaudhary</p>
			<p>8th May</p>
		</div>
	</div>
	<div class="blog">
		<h2>Title</h2>
		<p>Body of the blog</p>
		<div class="more-info">
			<p>321</p>
			<p>12</p>
			<p>Dev Chaudhary</p>
			<p>8th May</p>
		</div>
	</div>
</section>

<style>
	.blog-container {
		width: 100%;
		height: 100%;
		display: grid;
		grid-auto-flow: column;
		grid-template-columns: 4;
		background-color: var(--discord-blue);
	}

	.blog {
		margin: 2rem;
		background-color: var(--light);
		padding: 2rem;
		border-radius: 10px;
		max-height: 200px;
		color: var(--discord-blue--darkest);
		box-shadow: 5px 5px var(--discord-blue-dark);
	}

	.blog:hover {
		scale: 1.03;
		cursor: pointer;
	}

	.blog h2 {
		color: crimson;
	}
</style>
