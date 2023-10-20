<script lang="ts">
	export let value: any;
</script>

<div class="py-1 px-3 flex input-group">
	<input class="input mr-2" type="range" bind:value {...$$restProps} />
	<output>{value}</output>
</div>
