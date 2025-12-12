export function stringNotEmpty(value: string): boolean {
    return value.trim().length > 0;
}

export function stringMinLength(value: string, minLength: number): boolean {
    return value.trim().length >= minLength;
}

export function stringMaxLength(value: string, maxLength: number): boolean {
    return value.trim().length <= maxLength;
}

export function stringEqualsLength(value: string, exactLength: number): boolean {
    return value.trim().length === exactLength;
}

export function isValidUsernameLength(username: string): boolean {
  return /^[A-Za-z0-9]{3,16}$/.test(username);
}