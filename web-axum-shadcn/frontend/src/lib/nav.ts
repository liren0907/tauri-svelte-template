import type { Component } from 'svelte';
import LayoutGridIcon from '@lucide/svelte/icons/layout-grid';
import NotebookTextIcon from '@lucide/svelte/icons/notebook-text';

export interface NavItem {
	href: string;
	title: string;
	icon: Component;
}

/** Sidebar entries. Add a route under src/routes and list it here. */
export const navItems: NavItem[] = [
	{ href: '/', title: 'Notes', icon: NotebookTextIcon },
	{ href: '/showcase', title: 'UI Showcase', icon: LayoutGridIcon }
];

export function isActive(href: string, pathname: string): boolean {
	return href === '/' ? pathname === '/' : pathname === href || pathname.startsWith(`${href}/`);
}
