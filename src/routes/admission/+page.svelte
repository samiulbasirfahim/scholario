<script lang="ts">
	import InputContainer from '$lib/components/global/InputContainer.svelte';
	import Icon from '@iconify/svelte';
	import Modal from '$lib/components/admission/Modal.svelte';
	import type { Guardian } from '$lib/type/guardian';

	let classes = ['primary', 'secondery', 'intermediate', 'higher'];

	let selected_guardians: Guardian[] = $state([]);

	let admission_form = $state({
		name: '',
		phone: '',
		dob: '',
		address: '',
		religion: '',
		gender: '',
		previous_school: '',
		admission_date: '',
		blood_group: '',
		health_note: '',
		resident: false,
		class: '',
		section: '',
		photo: ''
	});

	$effect(() => console.log($state.snapshot(admission_form)));

	const remove_guardian = (id: number | undefined) => {
		selected_guardians = selected_guardians.filter((g) => g.id !== id);
	};

	const handleFormSubmit = () => {};
</script>

<form
	class="flex w-full flex-col p-8 lg:flex-row"
	onsubmit={(e) => {
		e.preventDefault();
		handleFormSubmit();
	}}
>
	<div class="flex w-full items-start justify-start">
		<div
			class="card bg-base-300 rounded-box grid grow grid-cols-1 place-items-center gap-4 p-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
		>
			<InputContainer name="name" label="Full name: ">
				<input type="text" name="name" class="input w-full" bind:value={admission_form.name} />
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
						bind:value={admission_form.phone}
					/>
					<p class="validator-hint">Must be 10 digits</p>
				</label>
			</InputContainer>

			<InputContainer name="prev_school" label="Previous school: ">
				<input
					type="text"
					name="prev_school"
					class="input w-full"
					bind:value={admission_form.previous_school}
				/>
			</InputContainer>

			<InputContainer name="blood-group" label="Blood group: ">
				<select class="select" id="blood-group" bind:value={admission_form.blood_group}>
					<option disabled selected value="">Blood group</option>
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

			<InputContainer
				name="health_note"
				label="Health note: "
				class="w-full sm:col-span-2 lg:col-span-1 2xl:col-span-2"
			>
				<input
					type="text"
					name="health_note"
					class="input w-full"
					bind:value={admission_form.health_note}
				/>
			</InputContainer>

			<InputContainer name="photo" label="Photo: ">
				<input
					type="file"
					accept="image/*"
					class="file-input file-input-primary w-full"
					name="photo"
					onchange={(e) => {
						let files = (e.target as HTMLInputElement).files;
						if (!files) return;

						let file = files[0];

						const reader = new FileReader();

						reader.onload = (ev) => {
							const base64String = ev.target.result.split(',')[1];
							admission_form.photo = 'data:image/png;base64,' + base64String;
						};

						reader.readAsDataURL(file);
					}}
				/>
			</InputContainer>

			<InputContainer name="dob" label="Date of birth: ">
				<input type="date" class="date input" name="dob" bind:value={admission_form.dob} />
			</InputContainer>

			<InputContainer
				name="address"
				label="Address: "
				class="w-full sm:col-span-2 lg:col-span-1 2xl:col-span-2"
			>
				<input
					type="text"
					name="address"
					class="input w-full"
					bind:value={admission_form.address}
				/>
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
	<div class="flex w-full items-start justify-start">
		<div
			class="card bg-base-300 rounded-box grid grow grid-cols-1 place-items-center gap-4 p-4 sm:grid-cols-2 lg:grid-cols-1 2xl:grid-cols-2"
		>
			<InputContainer name="class" label="Class: ">
				<select class="select" id="class" bind:value={admission_form.class}>
					<option disabled selected value="">Class</option>
					{#each classes as cls, i (cls)}
						<option value={i}>{cls}</option>
					{/each}
				</select>
			</InputContainer>
			<InputContainer name="section" label="Section: ">
				<select class="select" id="section" bind:value={admission_form.section}>
					<option disabled selected value="">Section</option>
					{#each classes as cls, i (cls)}
						<option value={i}>{cls}</option>
					{/each}
				</select>
			</InputContainer>

			<InputContainer name="monthly_fee" label="Monthly fee: ">
				<input type="number" class="input" name="monthly_fee" />
			</InputContainer>

			<InputContainer name="resident" label="Resident: ">
				<input type="checkbox" class="checkbox" bind:checked={admission_form.resident} />
			</InputContainer>

			{#if admission_form.resident}
				<InputContainer name="resident_fee" label="Resident fee: ">
					<input type="number" class="input" name="resident_fee" />
				</InputContainer>
			{/if}
			<input
				type="submit"
				class="btn btn-primary w-full sm:col-span-2 lg:col-span-1 2xl:col-span-2"
				value="Admit student"
			/>
		</div>
	</div>
</form>

<Modal bind:selected_guardians />
