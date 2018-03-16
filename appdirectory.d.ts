declare module 'appdirectory' {
    export default class AppDirectory {
        constructor(applicationName: string);

        userData(): string;

        userConfig(): string;

        userCache(): string;

        userLogs(): string;
    }
}
