<script lang="ts">
	const { children } = $props();
</script>

<div class="z-50 rounded flex bg-base-100  p-3">
	{@render children()}
</div>
