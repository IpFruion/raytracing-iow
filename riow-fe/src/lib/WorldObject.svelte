<script lang="ts">
	import Material from './Material.svelte';
	import Shape from './Shape.svelte';
	import type { WorldObjectData } from './index';

	export let value: WorldObjectData;
</script>

<div class="flex-row w-full m-3 pr-3 space-y-4">
	<Material bind:value={value.material} />
	<Shape bind:value={value.shape} />
</div>
