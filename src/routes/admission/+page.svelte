<script lang="ts">
	import InputContainer from '$lib/components/global/InputContainer.svelte';
	import Icon from '@iconify/svelte';
	import Modal from '$lib/components/admission/Modal.svelte';
	import type { Guardian } from '$lib/type/guardian';

	let admission_form = $state({
		name: '',
		phone: '',
		previous_school: '',
		address: '',
		class: '',
		section: '',
		blood_group: '',
		resident: '',
		photo: ''
	});

	let classes = ['primary', 'secondery', 'intermediate', 'higher'];

	let selected_guardians: Guardian[] = $state([]);

	const remove_guardian = (id: number | undefined) => {
		selected_guardians = selected_guardians.filter((g) => g.id !== id);
	};
</script>

<div class="flex w-full flex-col p-8 lg:flex-row">
	<div class="flex items-start justify-start">
		<div
			class="card bg-base-300 rounded-box grid grow grid-cols-1 place-items-center gap-4 p-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
		>
			<InputContainer name="name" label="Full name: ">
				<input type="text" name="name" class="input w-full" />
			</InputContainer>

			<InputContainer name="phone" label="Phone: ">
				<label class="input">
					+880
					<input
						type="tel"
						class="validator w-full grow tabular-nums"
						required
						pattern="[0-9]*"
						minlength="10"
						maxlength="10"
						title="Must be 10 digits"
					/>
					<p class="validator-hint">Must be 10 digits</p>
				</label>
			</InputContainer>

			<InputContainer name="name" label="Full name: ">
				<input type="text" name="name" class="input w-full" />
			</InputContainer>

			<InputContainer name="name" label="Full name: ">
				<input type="text" name="name" class="input w-full" />
			</InputContainer>

			<InputContainer name="name" label="Full name: ">
				<input type="text" name="name" class="input w-full" />
			</InputContainer>

			<InputContainer name="name" label="Full name: ">
				<input type="text" name="name" class="input w-full" />
			</InputContainer>

			<InputContainer
				name="address"
				label="Address: "
				class="w-full sm:col-span-2 lg:col-span-1 2xl:col-span-2"
			>
				<input type="text" name="address" class="input w-full" />
			</InputContainer>
			<ul class="bg-base-200 w-full sm:col-span-2 lg:col-span-1 2xl:col-span-2">
				<li class="p-4 pb-2 text-sm tracking-wide">Guardians</li>
				<div
					class="list card rounded-box grid max-h-54 w-full grow grid-cols-1 gap-4 overflow-y-auto px-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
				>
					{#each selected_guardians as guardian (guardian.id)}
						<li class="list-row bg-base-100 flex items-center justify-between">
							<div class="flex items-center justify-start">
								<div class="avatar">
									<div class="rounded-box w-16">
										<img src={guardian.photo} alt="{guardian.name}'s photo" />
									</div>
								</div>
								<div class="ps-2">
									<div class="text-xs font-light">{guardian.relation}</div>
									<div class="text-base font-semibold">{guardian.name}</div>
									<div class="text-sm font-medium uppercase">{guardian.phone}</div>
									<div class="text-xs font-light">{guardian.address}</div>
								</div>
							</div>
							<button class="btn btn-square btn-ghost" onclick={() => remove_guardian(guardian.id)}>
								<Icon icon="fa:remove" font-size="16" />
							</button>
						</li>
					{/each}
				</div>
				<div class="flex flex-wrap justify-between gap-4 p-4">
					<label for="create-guardian" class="btn btn-primary">Create Guardian</label>
					<label for="find-guardian" class="btn btn-primary">Find Guardian</label>
				</div>
			</ul>
		</div>
	</div>
	<div class="divider lg:divider-horizontal"></div>
	<div class="flex items-start justify-start">
		<div
			class="card bg-base-300 rounded-box grid grow grid-cols-1 place-items-center gap-4 p-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
		>
			<InputContainer name="class" label="Class: ">
				<select class="select" id="class">
					<option disabled selected>Class</option>
					{#each classes as cls (cls)}
						<option value={cls}>{cls}</option>
					{/each}
				</select>
			</InputContainer>

			<InputContainer name="section" label="Section: ">
				<select class="select" id="section">
					<option disabled selected>Section</option>
					{#each classes as cls (cls)}
						<option value={cls}>{cls}</option>
					{/each}
				</select>
			</InputContainer>

			<InputContainer name="blood-group" label="Blood group: ">
				<select class="select" id="blood-group">
					<option disabled selected>Blood group</option>
					<option value="A+">A+</option>
					<option value="A-">A-</option>
					<option value="B+">B+</option>
					<option value="B-">B-</option>
					<option value="AB+">AB+</option>
					<option value="AB-">AB-</option>
					<option value="O+">O+</option>
					<option value="O-">O-</option>
					<option value="unknown">Unknown</option>
				</select>
			</InputContainer>

			<InputContainer name="dob" label="Date of birth: ">
				<input type="date" class="input" name="dob" />
			</InputContainer>

			<InputContainer name="photo" label="Photo: ">
				<input type="file" class="file-input file-input-primary w-full" name="photo" />
			</InputContainer>

			<InputContainer name="resident" label="Resident">
				<input type="checkbox" checked class="checkbox" />
			</InputContainer>
		</div>
	</div>
</div>

<Modal bind:selected_guardians />
