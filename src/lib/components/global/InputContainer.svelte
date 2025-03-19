<script lang="ts">
	const { name, label, children, ...props } = $props();
</script>

<div class="w-full" {...props}>
	<div class="flex items-center justify-between">
		<label for={name} class="block text-sm/6 font-medium">{label}</label>
	</div>
	<div class="mt-2">
		{@render children()}
	</div>
</div>
