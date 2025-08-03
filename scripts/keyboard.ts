export type Keybindings = Map<string, () => void>;

export class KeyboardListener {
    public constructor(private keybindings: Keybindings) {
        this.startListener();
    }

    private heldKeys = new Set<string>();

    private startListener(): void {
        document.addEventListener("keydown", this.handleKeyDown.bind(this));
        document.addEventListener("keyup", this.handleKeyUp.bind(this));

        requestAnimationFrame(() => {
            this.tick();
        });
    }

    private handleKeyDown(event: KeyboardEvent): void {
        this.heldKeys.add(event.key.toLowerCase());
    }

    private handleKeyUp(event: KeyboardEvent): void {
        this.heldKeys.delete(event.key.toLowerCase());
    }

    private tick(): void {
        requestAnimationFrame(() => {
            this.heldKeys.forEach((key) => {
                const keyCallback = this.keybindings.get(key);
                keyCallback?.();
            });

            this.tick();
        });
    }
}
