<script lang="ts">
	import { Confirm } from '$lib/utility/Confirm';

	// function openDialog() {
	// 	dialogEl.showModal();
	//
	// 	dialogEl.addEventListener(
	// 		'close',
	// 		() => {
	// 			const result = dialogEl.returnValue;
	// 			console.log('Dialog result:', result === 'yes');
	// 		},
	// 		{ once: true }
	// 	);
	// }
</script>

<button
	on:click={() => {
		Confirm();
	}}>OPEN</button
>

<!-- <dialog bind:this={dialogEl}> -->
<!-- 	<p>Are you sure?</p> -->
<!-- 	<form method="dialog"> -->
<!-- 		<button value="yes">Yes</button> -->
<!-- 		<button value="no">No</button> -->
<!-- 	</form> -->
<!-- </dialog> -->
