<script lang="ts">
	import Slider from './Slider.svelte';
	import type { Material } from './index';
	export let value: Material;

	function updateSelected(e: any) {
		switch (e.target.value) {
			case 'lambertain': {
				value = { materialType: e.target.value, color: '#ffffff' };
				break;
			}
			case 'metal': {
				value = { materialType: e.target.value, color: '#ffffff', fuzziness: 0.0 };
				break;
			}
			case 'dielectric': {
				value = { materialType: e.target.value, indexOfRefraction: 1.5 };
				break;
			}
		}
	}
</script>

<div class="space-y-2">
	<h4 class="h4">Material</h4>
	<select class="select" on:change={updateSelected}>
		<option value="lambertain">Lambertain</option>
		<option value="dielectric">Dielectric</option>
		<option value="metal">Metal</option>
	</select>
	{#if value.materialType === 'lambertain'}
		<span>Color</span><br />
		<input class="input" type="color" bind:value={value.color} />
	{:else if value.materialType === 'metal'}
		<span>Color</span><br />
		<input class="input" type="color" bind:value={value.color} /><br />
		<span>Fuzziness</span>
		<Slider name="fuzziness" bind:value={value.fuzziness} min="0.0" max="1.0" step="0.1" />
	{:else if value.materialType === 'dielectric'}
		<span>Index of Refraction</span>
		<Slider
			name="index_of_refraction"
			bind:value={value.indexOfRefraction}
			min="0.1"
			max="5.0"
			step="0.1"
		/>
	{/if}
</div>
