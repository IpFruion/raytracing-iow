<script lang="ts">
	import type { Shape } from '$lib';
	import Vec3 from './Vec3.svelte';

	export let value: Shape;
	function updateSelected(e: any) {
		switch (e.target.value) {
			case 'sphere': {
				value = {
					shapeType: e.target.value,
					stationary: true,
					center: { x: 0.0, y: 0.0, z: 0.0 },
					radius: 1.0
				};
				break;
			}
		}
	}
</script>

<div class="space-y-2">
	<h4 class="h4">Shape</h4>
	<select class="select" on:change={updateSelected}>
		<option value="sphere">Sphere</option>
	</select>
	{#if value.shapeType === 'sphere'}
		{#if value.stationary}
			<span>Center</span>
			<Vec3 name="center" bind:value={value.center} />
			<span>Radius</span>
			<input class="input" type="number" bind:value={value.radius} />
		{/if}
	{/if}
</div>
