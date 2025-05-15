<script lang="ts">
	const { children } = $props();
</script>

<div class="navbar bg-base-100 z-50 rounded px-4 shadow-md">
	{@render children()}
</div>
