<script lang="ts">
	import Sidebar from '$lib/components/global/Sidebar.svelte';
	import Toast from '$lib/components/global/Toast.svelte';
	import { onMount } from 'svelte';
	import '../app.css';
	import { sessions } from '$lib/store/session.svelte';
	import { classes, sections } from '$lib/store/class.svelte';
	import { setting } from '$lib/store/setting.svelte';
	import { setDialogEl } from '$lib/utility/Confirm';

	let { children } = $props();

	onMount(() => {
		sessions.fetch();
		sections.fetch();
		setting.fetch();
	});

	$effect(() => {
		if (sessions.selectedSession?.id) classes.fetch(sessions.selectedSession.id);
	});
</script>

<Sidebar>
	{@render children()}
</Sidebar>

<Toast />

<dialog id="confirmation_modal" class="modal" use:setDialogEl>
	<form method="dialog" class="modal-box max-w-xs rounded-md p-6 space-y-5">
		<p class="font-semibold text-xl text-center">Are you sure?</p>
		<div class="flex justify-center gap-6">
			<button class="btn btn-error btn-sm rounded px-6 btn-outline" value="yes">Yes</button>
			<button class="btn btn-info btn-sm rounded px-6" value="no">No</button>
		</div>
	</form>
	<form method="dialog" class="modal-backdrop bg-base-100/60 blurred">
		<button aria-label="Close modal" class="h-full w-full" />
	</form>
</dialog>
