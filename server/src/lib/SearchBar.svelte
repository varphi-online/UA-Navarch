<script lang="ts">
	import Filter from 'lucide-svelte/icons/filter';
	import { grow } from '$lib/transitions';
	import * as ToggleGroup from '$lib/components/ui/toggle-group/index.js';
	import * as Select from '$lib/components/ui/select';
	import { CourseQuery, Course, Section, SectionQuery } from '$lib/query.svelte';
	import Input from './components/ui/input/input.svelte';
	import Combobox from './components/ui/combobox/combobox.svelte';
	import type { Writable } from 'svelte/store';
	import { getContext } from 'svelte';
	import type { QueryParams } from './queryStore.svelte';
	import Label from './components/ui/label/label.svelte';
	import Switch from './components/ui/switch/switch.svelte';
	import { fade } from 'svelte/transition';

	let {
		offset = $bindable(),
		limit = 15,
		loading = $bindable(false)
	}: { offset: number; limit?: number; loading: boolean } = $props();
	const queryParams: Writable<QueryParams> = getContext('queryParams');
	const queryResponse: { courses: Course[]; sections: Section[] } = getContext('queryResponse');
	let firstLoad = true;
	let activeFilters: string[] = $derived($queryParams.filters.map((filter) => filter.value));
	// AbortController for Debounce
	let currentController: AbortController | null = null;
	let searching: boolean = $derived(
		!!(
			($queryParams.desc && $queryParams.desc.length) ||
			($queryParams.dept && $queryParams.dept.length) ||
			($queryParams.num && $queryParams.num.length) ||
			$queryParams.attrs.length ||
			$queryParams.daysOfWeek.length ||
			($queryParams.instructor && $queryParams.instructor.length) ||
			($queryParams.class_num && $queryParams.class_num.length)
		)
	);

	let termState = $state({
		value: $queryParams.term,
		label: $queryParams.term
			.split(' ')
			.map((w) => w[0].toUpperCase() + w.substring(1).toLowerCase())
			.join(' ')
	});

	let selectedDepartment = $state('');

	// Query-Related
	let course_query: CourseQuery = $derived(
		new CourseQuery(
			activeFilters.includes('departments') ? $queryParams.dept : null,
			activeFilters.includes('course_number') ? $queryParams.num : null,
			null,
			activeFilters.includes('description') ? $queryParams.desc : null,
			activeFilters.includes('attributes') ? $queryParams.attrs.map((filter) => filter.value) : [],
			null,
			null,
			/*activeFilters.includes('term') ?*/ $queryParams.term /*: null*/,
			activeFilters.includes('showHist') ? true : false
		)
	);

	let section_query: SectionQuery = $derived(
		new SectionQuery(
			null,
			null,
			null,
			null,
			activeFilters.includes('instructor') ? $queryParams.instructor : null,
			activeFilters.includes('days') ? $queryParams.daysOfWeek : [],
			activeFilters.includes('times') ? $queryParams.startTime : null,
			activeFilters.includes('times') ? $queryParams.endTime : null,
			activeFilters.includes('class_id') ? $queryParams.class_num : null
		)
	);

	function highlight(text: string | null): string {
		// If text is null or undefined, return an empty string
		if (text == null) return '';
		// Check if description is too short
		if (!$queryParams.desc || $queryParams.desc.length < 4) return text;
		// Escape special regex characters in the description
		const escapedDesc = $queryParams.desc.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

		const regex = new RegExp(`(${escapedDesc})`, 'gi');
		return text.replace(regex, `<mark>$1</mark>`);
	}

	async function searchWithDebounce<T>(
		query: Record<string, any>,
		signal: AbortSignal,
		searchType: 'course' | 'section',
		debounceTime: number = 300
	): Promise<T[]> {
		loading = true;
		try {
			await new Promise((resolve, reject) => {
				const timeout = setTimeout(resolve, debounceTime);
				signal.addEventListener('abort', () => {
					clearTimeout(timeout);
					reject(/*new Error('Cancelled')*/);
				});
			});

			const response = await fetch('/api/search', {
				method: 'POST',
				body: JSON.stringify({ ...query, offset: offset, limit: limit }),
				headers: {
					'content-type': 'application/json',
					'search-type': searchType
				},
				signal
			});

			return await response.json();
		} catch (error) {
			if (error instanceof Error && error.name === 'AbortError') {
				// Silently handle aborted requests
				return [];
			}
			throw error; // Re-throw other errors
		}
	}

	// Search on args update
	$effect(() => {
		if (section_query || course_query) {
		}
		if (firstLoad) {
			firstLoad = false;
			return;
		}
		if (searching) {
			if (currentController) currentController.abort();
			let debounce_time = 300;
			if (
				offset &&
				$queryParams.desc &&
				$queryParams.desc.length < 6 &&
				$queryParams.attrs.length == 0
			) {
				debounce_time = 40 * $queryParams.desc.length;
			}

			// Create a new controller for this request
			currentController = new AbortController();

			if ($queryParams.searchType.value == 'course') {
				(async () => {
					const result = await searchWithDebounce<Course>(
						{ course: course_query },
						currentController.signal,
						'course',
						debounce_time
					);
					result.forEach((course) => (course.description = highlight(course.description)));
					offset == 0 ? (queryResponse.courses = result) : queryResponse.courses.push(...result);
					queryResponse.sections = [];
					loading = false;
				})();
			} else {
				(async () => {
					const result = await searchWithDebounce<Section>(
						{ section: section_query, course: course_query },
						currentController.signal,
						'section',
						debounce_time
					);
					offset == 0 ? (queryResponse.sections = result) : queryResponse.sections.push(...result);
					queryResponse.courses = [];
					loading = false;
				})();
			}
		} else {
			queryResponse.courses = [];
			queryResponse.sections = [];
		}
	});

	$effect(() => {
		// Reset limit on any query change
		if (
			$queryParams.desc ||
			$queryParams.dept ||
			$queryParams.num ||
			$queryParams.attrs ||
			$queryParams.daysOfWeek ||
			$queryParams.instructor
		)
			offset = 0;
	});
	$effect(() => {
		$queryParams.term = termState.value;
	});
</script>

<div class="box-content w-fit rounded-2xl border-2 border-solid border-gray-400 p-0">
	<div
		class=" z-50 flex h-full max-w-max flex-row flex-wrap items-center justify-center overflow-hidden [&>*]:z-40 [&>*]:h-9"
	>
		<Select.Root bind:selected={$queryParams.searchType}>
			<Select.Trigger
				class="m-0 h-full w-[100px] rounded-none rounded-bl-2xl rounded-tl-2xl border-y-0 border-l-0 border-r-[1px] "
			>
				<Select.Value placeholder="Courses" />
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="course">Courses</Select.Item>
				<Select.Item value="section">Sections</Select.Item>
			</Select.Content>
		</Select.Root>
		{#if activeFilters.includes('description')}<div class="h-min" transition:grow>
				<Input
					type="text"
					class=" w-min-2 m-0 h-full overflow-ellipsis rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
					bind:value={$queryParams.desc}
					placeholder="Keyword(s)"
				/>
			</div>{/if}
		{#if activeFilters.includes('departments')}
			<div class="h-fit" transition:grow>
				<Combobox
					options={[
						{ value: 'ABBS', label: 'ABBS - AZ Biological & Biomedical Sci' },
						{ value: 'ABS', label: 'ABS - Applied Biosciences' },
						{ value: 'ACBS', label: 'ACBS - Animal & Comp Biomed Sciences' },
						{ value: 'ACCT', label: 'ACCT - Accounting' },
						{ value: 'AED', label: 'AED - Agricultural Education Main' },
						{ value: 'AFAS', label: 'AFAS - Africana Studies Main' },
						{ value: 'AGTM', label: 'AGTM - Agricultural Technology Mgmt' },
						{ value: 'AIAR', label: 'AIAR - App Intercul Arts Res' },
						{ value: 'AIS', label: 'AIS - American Indian Studies' },
						{ value: 'AISG', label: 'AISG - American Indian Studies-GIDP' },
						{ value: 'ALC', label: 'ALC - Agricultural Leadership & Comm' },
						{ value: 'AME', label: 'AME - Aerospace & Mechanical Engr' },
						{ value: 'ANES', label: 'ANES - Anesthesiology' },
						{ value: 'ANTH', label: 'ANTH - Anthropology Main' },
						{ value: 'APAS', label: 'APAS - Asian Pacific American Studies' },
						{ value: 'APPL', label: 'APPL - Applied Mathematics' },
						{ value: 'ARB', label: 'ARB - Arabic' },
						{ value: 'ARC', label: 'ARC - Architecture' },
						{ value: 'ARCE', label: 'ARCE - Architectural Engineering' },
						{ value: 'ARE', label: 'ARE - Art Education' },
						{ value: 'AREC', label: 'AREC - Agric & Resource Economics' },
						{ value: 'ARH', label: 'ARH - Art History' },
						{ value: 'ARL', label: 'ARL - Arid Lands Resource Science' },
						{ value: 'ART', label: 'ART - Art' },
						{ value: 'ASTR', label: 'ASTR - Astronomy' },
						{ value: 'ATMO', label: 'ATMO - Atmospheric Sciences' },
						{ value: 'BAT', label: 'BAT - Biosystems Analytics & Tech' },
						{ value: 'BCOM', label: 'BCOM - Business Communications' },
						{ value: 'BE', label: 'BE - Biosystems Engineering' },
						{ value: 'BIOC', label: 'BIOC - Biochemistry' },
						{ value: 'BIOS', label: 'BIOS - Biostatistics' },
						{ value: 'BJP', label: 'BJP - Bilingual Journalism' },
						{ value: 'BME', label: 'BME - Biomedical Engineering' },
						{ value: 'BNAD', label: 'BNAD - Business Administration' },
						{ value: 'BNAN', label: 'BNAN - Business Analytics' },
						{ value: 'BSM', label: 'BSM - BS in Medicine' },
						{ value: 'CALS', label: 'CALS - Agriculture & Life Sciences' },
						{ value: 'CBIO', label: 'CBIO - Cancer Biology' },
						{ value: 'CE', label: 'CE - Civil Engineering' },
						{ value: 'CGSC', label: 'CGSC - Cognitive Sci, Non-GIDP' },
						{ value: 'CHEE', label: 'CHEE - Chemical & Environmental Engr' },
						{ value: 'CHEM', label: 'CHEM - Chemistry' },
						{ value: 'CHN', label: 'CHN - Chinese Studies' },
						{ value: 'CHS', label: 'CHS - Care, Health & Society' },
						{ value: 'CLAS', label: 'CLAS - Classics' },
						{ value: 'CMM', label: 'CMM - Cellular & Molecular Medicine' },
						{ value: 'COGS', label: 'COGS - Cognitive Science' },
						{ value: 'COMM', label: 'COMM - Communication' },
						{ value: 'CONS', label: 'CONS - Consortium' },
						{ value: 'COOP', label: 'COOP - Coop Work Phase' },
						{ value: 'CPH', label: 'CPH - Public Health' },
						{ value: 'CR', label: 'CR - Clinical Research' },
						{ value: 'CRL', label: 'CRL - Critical Languages' },
						{ value: 'CSC', label: 'CSC - Computer Science Main' },
						{ value: 'CTS', label: 'CTS - Clinical Translational Science' },
						{ value: 'CYBV', label: 'CYBV - Cyber Operations' },
						{ value: 'DATA', label: 'DATA - Statistics and Data Science' },
						{ value: 'DNC', label: 'DNC - Dance' },
						{ value: 'DVP', label: 'DVP - Development Practice' },
						{ value: 'EAS', label: 'EAS - East Asian Studies' },
						{ value: 'ECE', label: 'ECE - Electrical & Computer Engr' },
						{ value: 'ECOL', label: 'ECOL - Ecology & Evolutionary Biology' },
						{ value: 'ECON', label: 'ECON - Economics' },
						{ value: 'EDL', label: 'EDL - Educational Leadership' },
						{ value: 'EDP', label: 'EDP - Educational Psychology Main' },
						{ value: 'EHS', label: 'EHS - Environmental Health Sciences' },
						{ value: 'EIS', label: 'EIS - Entomology and Insect Science' },
						{ value: 'EM', label: 'EM - Engineering Mechanics' },
						{ value: 'EMD', label: 'EMD - Emergency Medicine' },
						{ value: 'ENGL', label: 'ENGL - English Main' },
						{ value: 'ENGR', label: 'ENGR - Engineering' },
						{ value: 'ENTO', label: 'ENTO - Entomology' },
						{ value: 'ENTR', label: 'ENTR - Entrepreneurship' },
						{ value: 'ENVS', label: 'ENVS - Environmental Science' },
						{ value: 'EPID', label: 'EPID - Epidemiology' },
						{ value: 'ESOC', label: 'ESOC - eSociety' },
						{ value: 'ETH', label: 'ETH - Biomedical Ethics' },
						{ value: 'EVAL', label: 'EVAL - Program Design and Evaluation' },
						{ value: 'EVS', label: 'EVS - Environmental Studies' },
						{ value: 'FA', label: 'FA - Fine Arts' },
						{ value: 'FCM', label: 'FCM - Family & Community Medicine' },
						{ value: 'FIN', label: 'FIN - Finance' },
						{ value: 'FITS', label: 'FITS - Fashion Industry Sci & Tech' },
						{ value: 'FOOD', label: 'FOOD - Food Studies' },
						{ value: 'FREN', label: 'FREN - French' },
						{ value: 'FTV', label: 'FTV - Film and Television' },
						{ value: 'GAME', label: 'GAME - Game Design, Dev., & Behavior' },
						{ value: 'GC', label: 'GC - Global Change' },
						{ value: 'GEN', label: 'GEN - Geological Engineering' },
						{ value: 'GENE', label: 'GENE - Genetics' },
						{ value: 'GEOG', label: 'GEOG - Geography & Development' },
						{ value: 'GEOS', label: 'GEOS - Geosciences' },
						{ value: 'GER', label: 'GER - German Studies' },
						{ value: 'GERO', label: 'GERO - Gerontology Main' },
						{ value: 'GHI', label: 'GHI - Global Health Institute' },
						{ value: 'GIST', label: 'GIST - Geographic Info Systems Tech' },
						{ value: 'GLO', label: 'GLO - Studies of Global Media' },
						{ value: 'GLS', label: 'GLS - Global Studies' },
						{ value: 'GRAD', label: 'GRAD - Graduate Studies' },
						{ value: 'GRK', label: 'GRK - Greek' },
						{ value: 'GWS', label: "GWS - Gender & Women's Studies" },
						{ value: 'HDFS', label: 'HDFS - Human Development & Family Sci' },
						{ value: 'HECL', label: 'HECL - Human Ecology' },
						{ value: 'HED', label: 'HED - Higher Education' },
						{ value: 'HIST', label: 'HIST - History Main' },
						{ value: 'HNRS', label: 'HNRS - Honors Studies' },
						{ value: 'HPS', label: 'HPS - Health Promotion Science' },
						{ value: 'HRTS', label: 'HRTS - Human Rights Practice' },
						{ value: 'HSD', label: 'HSD - Health Sciences Design' },
						{ value: 'HUMS', label: 'HUMS - Humanities' },
						{ value: 'HWRS', label: 'HWRS - Hydrology and Water Resources' },
						{ value: 'IA', label: 'IA - Instruction and Assessment' },
						{ value: 'IDS', label: 'IDS - Interdisciplinary Studies' },
						{ value: 'IIA', label: 'IIA - Innovations in Aging' },
						{ value: 'IMB', label: 'IMB - Immunobiology' },
						{ value: 'INFO', label: 'INFO - Information' },
						{ value: 'ISTA', label: 'ISTA - Information Sci, Tech & Arts' },
						{ value: 'ITAL', label: 'ITAL - Italian' },
						{ value: 'JOUR', label: 'JOUR - Journalism' },
						{ value: 'JPN', label: 'JPN - Japanese Studies' },
						{ value: 'JUS', label: 'JUS - Judaic Studies' },
						{ value: 'KOR', label: 'KOR - Korean' },
						{ value: 'LAR', label: 'LAR - Landscape Architecture' },
						{ value: 'LAS', label: 'LAS - Latin American Studies Main' },
						{ value: 'LAT', label: 'LAT - Latin' },
						{ value: 'LAW', label: 'LAW - Law' },
						{ value: 'LIBR', label: 'LIBR - Library Studies' },
						{ value: 'LING', label: 'LING - Linguistics' },
						{ value: 'LIS', label: 'LIS - Library & Information Science' },
						{ value: 'MAS', label: 'MAS - Mexican American Studies' },
						{ value: 'MATH', label: 'MATH - Mathematics Main' },
						{ value: 'MCB', label: 'MCB - Molecular & Cellular Biology' },
						{ value: 'MED', label: 'MED - Medicine: Interdept.' },
						{ value: 'MEDI', label: 'MEDI - Medicine' },
						{ value: 'MEDP', label: 'MEDP - Medicine Phoenix (Interdept)' },
						{ value: 'MENA', label: 'MENA - Middle East & N African St' },
						{ value: 'MGMT', label: 'MGMT - Management & Organizations' },
						{ value: 'MIBM', label: 'MIBM - Microbiome Core at Steele' },
						{ value: 'MIC', label: 'MIC - Microbiology' },
						{ value: 'MIN', label: 'MIN - Mining and Mineral Resources' },
						{ value: 'MIS', label: 'MIS - Management Info Systems Main' },
						{ value: 'MKTG', label: 'MKTG - Marketing' },
						{ value: 'MLA', label: 'MLA - Military Aerospace Studies' },
						{ value: 'MLS', label: 'MLS - Military Science' },
						{ value: 'MNE', label: 'MNE - Mining Engineering' },
						{ value: 'MSE', label: 'MSE - Materials Science & Engr' },
						{ value: 'MUS', label: 'MUS - Music Main' },
						{ value: 'MUSI', label: 'MUSI - Music Individual Studies' },
						{ value: 'NAFS', label: 'NAFS - Nutrition and Food Systems' },
						{ value: 'NEUR', label: 'NEUR - Neurology' },
						{ value: 'NROS', label: 'NROS - Neuroscience' },
						{ value: 'NRSC', label: 'NRSC - Neuroscience (Grad Program)' },
						{ value: 'NS', label: 'NS - Naval Science' },
						{ value: 'NSC', label: 'NSC - Nutritional Sciences' },
						{ value: 'NSCS', label: 'NSCS - Neuroscience & Cognitive Sci' },
						{ value: 'NSE', label: 'NSE - National Student Exchange' },
						{ value: 'NURS', label: 'NURS - Nursing' },
						{ value: 'OBG', label: 'OBG - Obstetrics & Gynecology' },
						{ value: 'OPH', label: 'OPH - Ophthalmology & Vision Science' },
						{ value: 'OPTI', label: 'OPTI - Optical Sciences' },
						{ value: 'ORTH', label: 'ORTH - Orthopaedic Surgery' },
						{ value: 'OSCM', label: 'OSCM - Operations & Supply Chain Mgmt' },
						{ value: 'OTO', label: 'OTO - Otolaryngology' },
						{ value: 'PA', label: 'PA - Public Administration & Policy' },
						{ value: 'PAH', label: 'PAH - Public and Applied Humanities' },
						{ value: 'PATH', label: 'PATH - Pathology' },
						{ value: 'PCOL', label: 'PCOL - Pharmacology & Toxicology' },
						{ value: 'PED', label: 'PED - Pediatrics' },
						{ value: 'PFFP', label: 'PFFP - Personal & Family Financial Pl' },
						{ value: 'PHCL', label: 'PHCL - Pharmacology' },
						{ value: 'PHIL', label: 'PHIL - Philosophy Main' },
						{ value: 'PHP', label: 'PHP - Public Health Practice' },
						{ value: 'PHPM', label: 'PHPM - Public Health Policy and Manag' },
						{ value: 'PHPR', label: 'PHPR - Pharmacy Practice' },
						{ value: 'PHSC', label: 'PHSC - Pharmaceutical Sciences' },
						{ value: 'PHYS', label: 'PHYS - Physics' },
						{ value: 'PLG', label: 'PLG - Planning' },
						{ value: 'PLP', label: 'PLP - Plant Pathology' },
						{ value: 'PLS', label: 'PLS - Plant Science' },
						{ value: 'POL', label: 'POL - Political Science Main' },
						{ value: 'PORT', label: 'PORT - Portuguese' },
						{ value: 'PPEL', label: 'PPEL - Phil, Pol, Econ & Law' },
						{ value: 'PR', label: 'PR - Public Relations' },
						{ value: 'PRS', label: 'PRS - Persian' },
						{ value: 'PS', label: 'PS - Physiological Sciences' },
						{ value: 'PSIO', label: 'PSIO - Physiology' },
						{ value: 'PSY', label: 'PSY - Psychology Main' },
						{ value: 'PSYI', label: 'PSYI - Psychiatry' },
						{ value: 'PSYS', label: 'PSYS - Psychological Science' },
						{ value: 'PTYS', label: 'PTYS - Planetary Sciences' },
						{ value: 'RADI', label: 'RADI - Radiology' },
						{ value: 'RAM', label: 'RAM - Range Management' },
						{ value: 'RCSC', label: 'RCSC - Retailing & Consumer Science' },
						{ value: 'RED', label: 'RED - Real Estate Development' },
						{ value: 'RELI', label: 'RELI - Religious Studies Main' },
						{ value: 'REM', label: 'REM - Remote Sensing' },
						{ value: 'RNR', label: 'RNR - Renewable Natural Resources' },
						{ value: 'RONC', label: 'RONC - Radiation Oncology' },
						{ value: 'RSSS', label: 'RSSS - Russian & Slavic Studies' },
						{ value: 'SA', label: 'SA - Study Abroad' },
						{ value: 'SAS', label: 'SAS - Student Affairs Studies' },
						{ value: 'SBE', label: 'SBE - Sustainable Built Environments' },
						{ value: 'SBS', label: 'SBS - Social and Behavioral Sciences' },
						{ value: 'SCI', label: 'SCI - Science' },
						{ value: 'SERP', label: 'SERP - Special Ed Rehab Sch Psyc Main' },
						{ value: 'SFWE', label: 'SFWE - Software Engineering' },
						{ value: 'SGPP', label: 'SGPP - Government & Public Policy' },
						{ value: 'SIE', label: 'SIE - Systems & Industrial Engr' },
						{ value: 'SLAT', label: 'SLAT - Scnd Lang Acq & Teaching' },
						{ value: 'SLHS', label: 'SLHS - Speech, Language & Hearing Sci' },
						{ value: 'SOC', label: 'SOC - Sociology Main' },
						{ value: 'SPAN', label: 'SPAN - Spanish Main' },
						{ value: 'SRL', label: 'SRL - Sport & Recreation Leadership' },
						{ value: 'STAT', label: 'STAT - Statistics' },
						{ value: 'STCH', label: 'STCH - Science Teaching' },
						{ value: 'SURG', label: 'SURG - Surgery' },
						{ value: 'TAR', label: 'TAR - Theatre Arts' },
						{ value: 'TFTV', label: 'TFTV - Theatre, Film & Television' },
						{ value: 'TLS', label: 'TLS - Tch, Lrn & Sociocultural Stdy' },
						{ value: 'TURK', label: 'TURK - Turkish' },
						{ value: 'UNIV', label: 'UNIV - University' },
						{ value: 'URO', label: 'URO - Urology' },
						{ value: 'VETM', label: 'VETM - Veterinary Medicine' },
						{ value: 'WFSC', label: 'WFSC - Wildlife & Fisheries Science' },
						{ value: 'WSM', label: 'WSM - Watershed Management' }
					]}
					bind:selected={$queryParams.dept}
					placeholder={'Department'}
					searchPlaceholder={'Search departments'}
				></Combobox>
			</div>{/if}
		{#if activeFilters.includes('course_number')}<div class="h-fit" transition:grow>
				<Input
					type="text"
					class=" m-0 h-full w-[7.7rem] rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
					bind:value={$queryParams.num}
					placeholder="Course Number"
				/>
			</div>{/if}
		{#if activeFilters.includes('class_id') && $queryParams.searchType.value == 'section'}<div
				class="h-fit"
				transition:grow
			>
				<Input
					type="text"
					class=" m-0 h-full w-[7.4rem] rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
					bind:value={$queryParams.class_num}
					placeholder="Class ID"
				/>
			</div>{/if}
		{#if activeFilters.includes('instructor') && $queryParams.searchType.value == 'section'}<div
				class="h-fit"
				transition:grow
			>
				<Input
					type="text"
					class=" m-0 h-full rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300 focus-visible:ring-transparent focus-visible:ring-offset-0"
					bind:value={$queryParams.instructor}
					placeholder="Instructor"
				/>
			</div>{/if}

		{#if activeFilters.includes('attributes')}<div class="h-fit" transition:grow>
				<Select.Root multiple bind:selected={$queryParams.attrs}>
					<Select.Trigger
						class="border-gray-30  m-0 h-full overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px]"
					>
						<Select.Value class="overflow-hidden" placeholder="Course Attribute(s)" />
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="bc">Building Connections</Select.Item>
						<Select.Item value="hum">EP: Humanist</Select.Item>
						<Select.Item value="art">EP: Artist</Select.Item>
						<Select.Item value="ns">EP: Natural Scientist</Select.Item>
						<Select.Item value="ss">EP: Social Scientist</Select.Item>
						<Select.Item value="ec">Entry Course</Select.Item>
						<Select.Item value="xc">Exit Course</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
		{/if}
		{#if activeFilters.includes('days') && $queryParams.searchType.value == 'section'}
			<div class="h-full" transition:grow>
				<ToggleGroup.Root
					type="multiple"
					bind:value={$queryParams.daysOfWeek}
					class=" m-0 h-full overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300"
				>
					<ToggleGroup.Item class="h-full" value="mo" aria-label="Toggle Monday"
						>Mo</ToggleGroup.Item
					>
					<ToggleGroup.Item class="h-full" value="tu" aria-label="Toggle Tuesday"
						>Tu</ToggleGroup.Item
					>
					<ToggleGroup.Item class="h-full" value="we" aria-label="Toggle Wednesday"
						>We</ToggleGroup.Item
					>
					<ToggleGroup.Item class="h-full" value="th" aria-label="Toggle Thursday"
						>Th</ToggleGroup.Item
					>
					<ToggleGroup.Item class="h-full" value="fr" aria-label="Toggle Friday"
						>Fr</ToggleGroup.Item
					>
				</ToggleGroup.Root>
			</div>
		{/if}
		{#if activeFilters.includes('term')}<div class="h-fit" transition:grow>
				<Select.Root bind:selected={termState}>
					<Select.Trigger
						class="border-gray-30  m-0 h-full overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px]"
					>
						<Select.Value class="overflow-hidden" placeholder="Course Attribute(s)" />
					</Select.Trigger>
					<Select.Content>
						<Select.Item value="Spring 2025">Spring 2025</Select.Item>
						<Select.Item value="Summer 2025">Summer 2025</Select.Item>
						<Select.Item value="Fall 2025">Fall 2025</Select.Item>
					</Select.Content>
				</Select.Root>
			</div>
		{/if}
		{#if activeFilters.includes('times') && $queryParams.searchType.value == 'section'}
			<div
				class="m-0 flex h-full flex-row items-center overflow-hidden rounded-none border-y-0 border-l-0 border-r-[1px] border-gray-300"
				transition:grow
			>
				<div>
					<!--<Label for="st">Start time</Label>-->
					<Input
						bind:value={$queryParams.startTime}
						class="h-full border-0"
						type="time"
						id="st"
						placeholder="08:00"
						autocomplete="off"
					/>
				</div>
				<p>-</p>
				<div>
					<!--<Label for="et">End time</Label>-->
					<Input
						bind:value={$queryParams.endTime}
						class="h-full border-0"
						type="time"
						id="et"
						placeholder="18:00"
						autocomplete="off"
					/>
				</div>
			</div>
		{/if}

		<Select.Root multiple bind:selected={$queryParams.filters}>
			<Select.Trigger class="h-full w-10 rounded-br-2xl rounded-tr-2xl border-none [&>*]:hidden">
				<Filter class="!block h-7 w-7 opacity-50" />
			</Select.Trigger>
			<Select.Content>
				<Select.Item value="description">Keywords</Select.Item>
				<Select.Item value="departments">Department</Select.Item>
				<Select.Item value="course_number">Course Number</Select.Item>
				<Select.Item value="attributes">Course Attributes</Select.Item>
				{#if $queryParams.searchType.value == 'section'}
					<Select.Item value="days">Week Days</Select.Item>
					<Select.Item value="times">Times</Select.Item>
					<Select.Item value="instructor">Instructor</Select.Item>
					<Select.Item value="class_id">Class ID</Select.Item>
				{/if}
				<Select.Item value="term">Term</Select.Item>
				<Select.Item value="showHist">Show History</Select.Item>
				
			</Select.Content>
		</Select.Root>
	</div>
</div>
{#if $queryParams.searchType.value == 'section'}
				<div transition:fade class="flex justify-center items-center gap-2 -mt-5">
					<Switch id="showOpenClasses" bind:checked={$queryParams.showOpen} />
					<Label for="showOpenClasses">Open classes only</Label>
				</div>
{/if}