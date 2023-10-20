<script lang="ts">
	import Slider from './Slider.svelte';
	import Vec3 from './Vec3.svelte';
	import WorldObject from './WorldObject.svelte';
	import { defaultGenerateImageRequest } from './generateImage';
	import type { GenerateImageRequest } from './generateImage';
	import { defaultWorldObjectData } from './index';

	export let value: GenerateImageRequest = defaultGenerateImageRequest();

	export let onSubmit: (value: GenerateImageRequest) => void;

	function addObject() {
		value.world = [...value.world, defaultWorldObjectData()];
	}

	function removeObject(i: number) {
		value.world = value.world.filter((_, idx) => idx !== i);
	}
</script>

<form class="space-y-6" on:submit={(e) => e.preventDefault()}>
	<h3 class="h3">Image Config</h3>
	<div class="flex flex-wrap -mx-3">
		<label class="label w-full md:w-1/2 px-3">
			<span>Width</span>
			<input
				name="width"
				class="input"
				type="number"
				bind:value={value.imageConfig.width}
				min="10"
				max="3000"
			/>
		</label>
		<label class="label w-full md:w-1/2 px-3">
			<span>Height</span>
			<input
				name="height"
				class="input"
				type="number"
				bind:value={value.imageConfig.height}
				min="10"
				max="3000"
			/>
		</label>
	</div>
	<h3 class="h3">Camera Config</h3>
	<div>
		<div class="flex flex-wrap -mx-3 mb-3">
			<label class="label w-full md:w-1/3 px-3" for="samples_per_pixel">
				<span>Samples Per Pixel</span>
				<Slider
					name="samples_per_pixel"
					bind:value={value.cameraConfig.samplesPerPixel}
					min="10"
					max="500"
					step="10"
				/>
			</label>
			<label class="label w-full md:w-1/3 px-3" for="max_depth">
				<span>Max Ray Bounce Depth</span>
				<Slider
					name="max_depth"
					bind:value={value.cameraConfig.maxDepth}
					min="1"
					max="50"
					step="1"
				/>
			</label>
			<label class="label w-full md:w-1/3 px-3" for="defocus_angle">
				<span>Defocus Angle</span>
				<Slider
					name="defocus_angle"
					bind:value={value.cameraConfig.defocusAngle}
					min="0.1"
					max="1.0"
					step="0.1"
				/>
			</label>
		</div>
		<h5 class="h5">Position</h5>
		<Vec3 name="position" bind:value={value.cameraConfig.position} />
		<h5 class="h5">Up</h5>
		<Vec3 name="up" bind:value={value.cameraConfig.up} />
		<h5 class="h5">Look At</h5>
		<Vec3 name="look_at" bind:value={value.cameraConfig.lookAt} />
	</div>
	<h3 class="h3">World Config</h3>
	<div>
		<button class="btn variant-filled-secondary" on:click={addObject}>Add Object</button>
		<ul class="list w-full mt-3 space-y-3">
			{#each value.world as worldObj, i}
				<li class="content-center flex-row rounded-md bg-gray-700 px-3">
					<WorldObject value={worldObj} />
					<button class="btn self-center variant-filled-warning" on:click={() => removeObject(i)}
						>Remove</button
					>
				</li>
			{/each}
		</ul>
	</div>
	<button class="btn variant-filled-primary" on:click={() => onSubmit(value)}>Generate</button>
</form>
