<script lang="ts">
	import { Search, ChevronsUpDown, Check } from 'lucide-svelte';
	import * as Popover from '$lib/components/ui/popover';
	import { Button } from '$lib/components/ui/button';
	import Input from '../input/input.svelte';
	import { cn } from '$lib/utils.js';
	import { tick } from 'svelte';

	let {
		options = [],
		selected = $bindable(),
		placeholder = '',
		searchPlaceholder = ''
	}: {
		options: { value: string; label: string }[];
		selected: string;
		placeholder: string;
		searchPlaceholder: string;
	} = $props();

	let open = $state(false);
	let search = $state('');
	let selectedLabel = $state(options.find(option=>option.value===selected)?.label??"");

    let filteredOptions = $derived.by(()=>{
        let lowerSearch = search.toLowerCase();
        return options.filter(option=>option.label.toLowerCase().includes(lowerSearch))});

	// We want to refocus the trigger button when the user selects
	// an item from the list so users can continue navigating the
	// rest of the form with the keyboard.
	function closeAndFocusTrigger(triggerId: string) {
		open = false;
		tick().then(() => {
			document.getElementById(triggerId)?.focus();
		});
	}
</script>

<Popover.Root bind:open let:ids>
	<Popover.Trigger asChild let:builder>
		<Button
			builders={[builder]}
			variant="outline"
			role="combobox"
			aria-expanded={open}
            style={selectedLabel.length ? "color: black;"
            :"--tw-text-opacity: 1; color: rgb(107 114 128 / var(--tw-text-opacity))"
            }
			class="border-gray-30 m-0 h-full w-min overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px] font-normal"
		>
			{selectedLabel.length ? selectedLabel : placeholder}
			<ChevronsUpDown class="ml-2 h-4 w-4 shrink-0 opacity-50" />
		</Button>
	</Popover.Trigger>
	<Popover.Content class="w-min p-0">
		<div class="flex items-center border-b px-2" data-cmdk-input-wrapper="">
			<Search class="mr-2 h-4 w-4 shrink-0 opacity-50" />
			<Input
				type="text"
				bind:value={search}
				placeholder={searchPlaceholder}
				class="flex h-11 min-w-[7.5rem] max-w-full rounded-md border-none bg-transparent p-0 py-3 text-sm outline-none placeholder:text-muted-foreground focus-visible:ring-transparent focus-visible:ring-offset-0 disabled:cursor-not-allowed disabled:opacity-50"
			/>
		</div>
		<div class="max-h-[300px] overflow-y-auto">
			{#each filteredOptions as option}
					<Button
                        class="bg-white text-black w-full justify-start font-normal hover:bg-slate-100"
						on:click={() => {
							({ value: selected, label: selectedLabel } =
								option.label === selectedLabel
									? {
											value: '',
											label: placeholder
										}
									: (filteredOptions.find((f) => f.label === option.label) ?? {
											value: '',
											label: placeholder
										}));
							closeAndFocusTrigger(ids.trigger);
						}}
					>
						<Check
							class={cn(
								'mr-2 h-4 w-4 text-lg',
								selectedLabel !== option.label && 'text-transparent'
							)}
						/>
						{option.label}</Button
					>
			{/each}
		</div>
	</Popover.Content>
</Popover.Root>
