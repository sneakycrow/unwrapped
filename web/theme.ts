import type { CustomThemeConfig } from '@skeletonlabs/tw-plugin';

export const sneakyCrowSkeletonTheme: CustomThemeConfig = {
	name: 'sneakycrow-skeleton',
	properties: {
		// =~= Theme Properties =~=
		'--theme-font-family-base': `Outfit, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		'--theme-font-family-heading': `Outfit, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans', sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'`,
		'--theme-font-color-base': '0 0 0',
		'--theme-font-color-dark': '255 255 255',
		'--theme-rounded-base': '12px',
		'--theme-rounded-container': '6px',
		'--theme-border-base': '4px',
		// =~= Theme On-X Colors =~=
		'--on-primary': '0 0 0',
		'--on-secondary': '0 0 0',
		'--on-tertiary': '0 0 0',
		'--on-success': '0 0 0',
		'--on-warning': '0 0 0',
		'--on-error': '0 0 0',
		'--on-surface': '255 255 255',
		// =~= Theme Colors  =~=
		// primary | #0ba750
		'--color-primary-50': '218 242 229', // #daf2e5
		'--color-primary-100': '206 237 220', // #ceeddc
		'--color-primary-200': '194 233 211', // #c2e9d3
		'--color-primary-300': '157 220 185', // #9ddcb9
		'--color-primary-400': '84 193 133', // #54c185
		'--color-primary-500': '11 167 80', // #0ba750
		'--color-primary-600': '10 150 72', // #0a9648
		'--color-primary-700': '8 125 60', // #087d3c
		'--color-primary-800': '7 100 48', // #076430
		'--color-primary-900': '5 82 39', // #055227
		// secondary | #f0f66e
		'--color-secondary-50': '253 254 233', // #fdfee9
		'--color-secondary-100': '252 253 226', // #fcfde2
		'--color-secondary-200': '251 253 219', // #fbfddb
		'--color-secondary-300': '249 251 197', // #f9fbc5
		'--color-secondary-400': '245 249 154', // #f5f99a
		'--color-secondary-500': '240 246 110', // #f0f66e
		'--color-secondary-600': '216 221 99', // #d8dd63
		'--color-secondary-700': '180 185 83', // #b4b953
		'--color-secondary-800': '144 148 66', // #909442
		'--color-secondary-900': '118 121 54', // #767936
		// tertiary | #f0f8ea
		'--color-tertiary-50': '253 254 252', // #fdfefc
		'--color-tertiary-100': '252 254 251', // #fcfefb
		'--color-tertiary-200': '251 253 250', // #fbfdfa
		'--color-tertiary-300': '249 252 247', // #f9fcf7
		'--color-tertiary-400': '245 250 240', // #f5faf0
		'--color-tertiary-500': '240 248 234', // #f0f8ea
		'--color-tertiary-600': '216 223 211', // #d8dfd3
		'--color-tertiary-700': '180 186 176', // #b4bab0
		'--color-tertiary-800': '144 149 140', // #90958c
		'--color-tertiary-900': '118 122 115', // #767a73
		// success | #00A5CF
		'--color-success-50': '217 242 248', // #d9f2f8
		'--color-success-100': '204 237 245', // #ccedf5
		'--color-success-200': '191 233 243', // #bfe9f3
		'--color-success-300': '153 219 236', // #99dbec
		'--color-success-400': '77 192 221', // #4dc0dd
		'--color-success-500': '0 165 207', // #00A5CF
		'--color-success-600': '0 149 186', // #0095ba
		'--color-success-700': '0 124 155', // #007c9b
		'--color-success-800': '0 99 124', // #00637c
		'--color-success-900': '0 81 101', // #005165
		// warning | #E6AF2E
		'--color-warning-50': '251 243 224', // #fbf3e0
		'--color-warning-100': '250 239 213', // #faefd5
		'--color-warning-200': '249 235 203', // #f9ebcb
		'--color-warning-300': '245 223 171', // #f5dfab
		'--color-warning-400': '238 199 109', // #eec76d
		'--color-warning-500': '230 175 46', // #E6AF2E
		'--color-warning-600': '207 158 41', // #cf9e29
		'--color-warning-700': '173 131 35', // #ad8323
		'--color-warning-800': '138 105 28', // #8a691c
		'--color-warning-900': '113 86 23', // #715617
		// error | #D64933
		'--color-error-50': '249 228 224', // #f9e4e0
		'--color-error-100': '247 219 214', // #f7dbd6
		'--color-error-200': '245 210 204', // #f5d2cc
		'--color-error-300': '239 182 173', // #efb6ad
		'--color-error-400': '226 128 112', // #e28070
		'--color-error-500': '214 73 51', // #D64933
		'--color-error-600': '193 66 46', // #c1422e
		'--color-error-700': '161 55 38', // #a13726
		'--color-error-800': '128 44 31', // #802c1f
		'--color-error-900': '105 36 25', // #692419
		// surface | #1f1f1f
		'--color-surface-50': '221 221 221', // #dddddd
		'--color-surface-100': '210 210 210', // #d2d2d2
		'--color-surface-200': '199 199 199', // #c7c7c7
		'--color-surface-300': '165 165 165', // #a5a5a5
		'--color-surface-400': '98 98 98', // #626262
		'--color-surface-500': '31 31 31', // #1f1f1f
		'--color-surface-600': '28 28 28', // #1c1c1c
		'--color-surface-700': '23 23 23', // #171717
		'--color-surface-800': '19 19 19', // #131313
		'--color-surface-900': '15 15 15' // #0f0f0f
	}
};
