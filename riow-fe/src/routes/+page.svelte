<script lang="ts">
	import { ProgressBar } from '@skeletonlabs/skeleton';
	import { generateImage } from '$lib/generateImage';
	import type { GenerateImageRequest, GenerateImageResponse } from '$lib/generateImage';
	import { imageGenStatus, downloadImage } from '$lib/index';
	import type { ImageStatus } from '$lib/index';
	import GenerateForm from '$lib/GenerateForm.svelte';

	let result: GenerateImageResponse | null = null;
	let statusUpdater: NodeJS.Timeout | null = null;
	let status: ImageStatus | null = null;
	let downloadUrl: string | null = null;

	function clearUpdateImageStatus() {
		if (statusUpdater) {
			clearInterval(statusUpdater);
		}
		statusUpdater = null;
		result = null;
	}

	async function updateImageStatus() {
		if (result == null) {
			clearUpdateImageStatus();
			return;
		}
		status = await imageGenStatus(result.status_url).catch((e) => {
			clearUpdateImageStatus();
			throw e;
		});
		if (status.download_url) {
			clearUpdateImageStatus();
			downloadUrl = await downloadImage(status.download_url);
		} else if (statusUpdater == null) {
			statusUpdater = setInterval(updateImageStatus, 500);
		}
	}

	async function onPress(value: GenerateImageRequest) {
		result = await generateImage(value);
		await updateImageStatus();
	}
</script>

<div class="container mx-auto p-8 space-y-8">
	<h1 class="h1">Raytracing in One Weekend (in Rust 🦀)</h1>
	<h4 class="h4">
		<a class="anchor" href="https://github.com/IpFruion/raytracing-iow">impl</a> by Derrick Lockwood
	</h4>
	<p>
		This is an implementation of <a class="anchor" href="https://raytracing.github.io/"
			>Raytracing in One Weekend</a
		>
		in Rust. Feel free to try it out, look at the code, and / or explore. The swagger docs are located
		<a class="anchor" href="/swagger-ui">here</a>
	</p>
	{#if status == null}
		<GenerateForm onSubmit={onPress} />
	{:else if status?.Rendering}
		<ProgressBar
			label="Rendering"
			value={status.Rendering.cur_pixel}
			max={status.Rendering.max_pixels}
		/>
	{:else if downloadUrl}
		<img src={downloadUrl} alt="Preview" />
		<a class="btn variant-filled-success" href={downloadUrl} download="raytracing-iow.png"
			>Download Image</a
		>
	{:else}
		<h2 class="h2">Queueing</h2>
	{/if}
</div>
