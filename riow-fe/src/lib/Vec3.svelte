<script lang="ts">
	import type { Vec3 } from './index';
	export let name: string;
	export let value: Vec3;
</script>

<div class="flex">
	<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] w-full md:w-1/3 m-2">
		<div class="input-group-shim">x</div>
		<input name={name.concat('.x')} class="input" type="number" bind:value={value.x} />
	</div>
	<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] w-full md:w-1/3 m-2">
		<div class="input-group-shim">y</div>
		<input name={name.concat('.y')} class="input" type="number" bind:value={value.y} />
	</div>
	<div class="input-group input-group-divider grid-cols-[auto_1fr_auto] w-full md:w-1/3 m-2">
		<div class="input-group-shim">x</div>
		<input name={name.concat('.z')} class="input" type="number" bind:value={value.z} />
	</div>
</div>
