import type { Issue } from '$lib/tauri';

/**
 * Human-readable location for a validation issue: the file, then the module
 * and field ids when present, separated by middle dots.
 */
export function issueLocation(issue: Issue): string {
    const parts = [issue.file];
    if (issue.module_id) parts.push(issue.module_id);
    if (issue.field_id) parts.push(issue.field_id);
    return parts.join(' · ');
}
