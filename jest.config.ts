import type Jest from '@jest/types';

const config: Jest.Config.InitialOptions = {
  moduleFileExtensions: ['js', 'json', 'ts'],
  testRegex: '.*\\.(spec|test)\\.ts$',
  transform: {
    '^.+\\.(t|j)s$': 'ts-jest',
  },
  collectCoverageFrom: ['src/**/*.ts'],
  coveragePathIgnorePatterns: ['main\\.ts', '.*\\.(spec|test)\\.helper\\.ts$'],
  coverageDirectory: '.coverage',
  testEnvironment: 'node',
  setupFiles: [],

  // in debug environment debugPort is almost always not a default value
  // but in non debug it is always 9229, so increase timeout for local debugging
  testTimeout: process.debugPort === 9229 ? 9999999 : 15000,
};

export default config;
