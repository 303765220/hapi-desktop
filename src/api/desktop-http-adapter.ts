import axios, {
  AxiosError,
  AxiosHeaders,
  type AxiosAdapter,
  type AxiosResponse,
  type InternalAxiosRequestConfig,
} from 'axios'

type TauriFetch = typeof import('@tauri-apps/plugin-http').fetch

function hasRequestBody(method: string): boolean {
  return method !== 'GET' && method !== 'HEAD'
}

function normalizeHeaders(config: InternalAxiosRequestConfig): Record<string, string> {
  const headers = new AxiosHeaders(config.headers).toJSON()
  return Object.fromEntries(
    Object.entries(headers)
      .filter(([, value]) => value !== null && value !== undefined)
      .map(([key, value]) => [key, String(value)])
  )
}

async function readResponseData(response: Response, responseType: string | undefined): Promise<unknown> {
  if (responseType === 'arraybuffer') {
    return response.arrayBuffer()
  }
  if (responseType === 'blob') {
    return response.blob()
  }

  return response.text()
}

function buildAxiosResponse(
  config: InternalAxiosRequestConfig,
  response: Response,
  data: unknown
): AxiosResponse {
  return {
    data,
    status: response.status,
    statusText: response.statusText,
    headers: Object.fromEntries(response.headers.entries()),
    config,
    request: null,
  }
}

async function loadTauriFetch(): Promise<TauriFetch> {
  const { fetch } = await import('@tauri-apps/plugin-http')
  return fetch
}

export function createDesktopHttpAdapter(loadFetch: () => Promise<TauriFetch> = loadTauriFetch): AxiosAdapter {
  return async (config) => {
    const fetch = await loadFetch()
    const method = (config.method || 'get').toUpperCase()
    const url = axios.getUri(config)
    const body = hasRequestBody(method) ? config.data : undefined

    const response = await fetch(url, {
      method,
      headers: normalizeHeaders(config),
      body,
      signal: config.signal as AbortSignal | undefined,
      connectTimeout: config.timeout || undefined,
    })
    const data = await readResponseData(response, config.responseType)
    const axiosResponse = buildAxiosResponse(config, response, data)
    const validateStatus = config.validateStatus || ((status: number) => status >= 200 && status < 300)

    if (!validateStatus(response.status)) {
      throw new AxiosError(
        `Request failed with status code ${response.status}`,
        AxiosError.ERR_BAD_REQUEST,
        config,
        null,
        axiosResponse
      )
    }

    return axiosResponse
  }
}
