/* [commitlint](https://github.com/conventional-changelog/commitlint) configuration */
import {
    RuleConfigSeverity,
} from '@commitlint/types';

export default {
    parserPreset: 'conventional-changelog-conventionalcommits',
    rules: {
        'header-max-length': [RuleConfigSeverity.Error, 'always', 72], // Header should be 72 characters or shorter
        'header-trim': [RuleConfigSeverity.Error, 'always'], // No leading/trailing whitespace in header
        'subject-empty': [RuleConfigSeverity.Error, 'never'], // No empty subject
        'subject-case': [ // Subject line should be lowercase
            RuleConfigSeverity.Error,
            'never',
            ['sentence-case', 'start-case', 'pascal-case', 'upper-case']],
        'subject-full-stop': [RuleConfigSeverity.Error, 'never'], // No full-stop at end of subject
        'body-max-line-length': [RuleConfigSeverity.Error, 'always', 72], // Body lines should be 72 characteres or shorter
        'body-leading-blank': [RuleConfigSeverity.Error, 'always'], // Empty line before body
        'type-empty': [RuleConfigSeverity.Error, 'never'], // Commit type must be present
        'type-case': [RuleConfigSeverity.Error, 'always', 'lower-case'], // Commit type should be lowercase
        'type-enum': [ // Commit type allowlist
            RuleConfigSeverity.Error,
            'always',
            [
                'build',
                'chore',
                'ci',
                'docs',
                'feat',
                'fix',
                'perf',
                'refactor',
                'revert',
                'style',
                'test',
            ],
        ],
    },
    ignores: [
        (message) => message.includes("Merge pull request #"), // PR merges are allowed
    ],
};
