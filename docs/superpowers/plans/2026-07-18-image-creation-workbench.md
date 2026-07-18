# 图片创作工作台 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把当前依赖 Infinite Canvas 的「图片创作」页改成 Hapi 自研的图片创作工作台，支持提示词、参考图、参数、结果和历史，并与站内 Key 体系联动。

**Architecture:** 保留现有 `/image-creation` 路由和菜单名，只替换页面内部实现。先把图片生成 API 契约和请求组装单独收口到 `src/api/image-creation.ts`，再用一个小的 service/composable 管理默认 Key、表单校验、任务提交和轮询，最后把 `ImageCreationView.vue` 改成响应式工作台页面。这样 backend 变化只会落在 API 层，页面层只关心状态和展示。

**Tech Stack:** Vue 3、TypeScript、Axios `apiClient`、Vitest、现有 Hapi 站内 Key API、Tauri 桌面端前端壳。

## Global Constraints

- 现有 `/image-creation` 路由和左侧「图片创作」菜单名保留，只替换内部实现。
- 不做无限画布。
- 不做图层编辑器。
- 不做自由绘制、圈选、涂抹式画布。
- 不做模板排版工具。
- 不依赖外部 Infinite Canvas 页面。
- 参考图最多允许 4 张。
- 前端只传 `key_id`，不把原始密钥发给第三方页面。
- Web 端和桌面端共用同一套页面。
- 最终页面需要通过本地构建验证，并在真实站内 Key 下做一次端到端生成检查。

---

### Task 1: Define the image-generation API contract

**Files:**
- Create: `src/types/image-creation.ts`
- Create: `src/api/image-creation.ts`
- Modify: `src/api/index.ts`
- Create: `src/api/__tests__/image-creation.spec.ts`

**Interfaces:**
- Consumes: existing `apiClient` from `src/api/client.ts`.
- Produces: `ImageCreationDraft`, `ImageCreationJob`, `ImageCreationHistoryItem`, `buildImageGenerationFormData()`, `createImageGenerationJob()`, `getImageGenerationJob()`, `listImageGenerationJobs()`, `resolveImageDownloadUrl()`.

- [ ] **Step 1: Write the failing test**

```ts
it('builds a multipart payload with prompt, key id, model, and reference images', () => {
  const formData = buildImageGenerationFormData({
    keyId: 12,
    prompt: 'a cat in a blue coat',
    negativePrompt: '',
    model: 'gpt-image-1',
    aspectRatio: '1:1',
    count: 1,
    referenceImages: [createReferenceImage('ref.png')]
  })

  expect(formData.get('key_id')).toBe('12')
  expect(formData.get('prompt')).toBe('a cat in a blue coat')
  expect(formData.get('model')).toBe('gpt-image-1')
  expect(formData.getAll('reference_images')).toHaveLength(1)
})
```

- [ ] **Step 2: Run test to verify it fails**

Run: `npx vitest run src/api/__tests__/image-creation.spec.ts`

Expected: FAIL because `src/api/image-creation.ts` and the related types do not exist yet.

- [ ] **Step 3: Write minimal implementation**

Implement a small API module that keeps every image-generation field in one place, uses `FormData` for reference uploads, and wraps the backend contract behind named functions.

- [ ] **Step 4: Run test to verify it passes**

Run: `npx vitest run src/api/__tests__/image-creation.spec.ts`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/types/image-creation.ts src/api/image-creation.ts src/api/index.ts src/api/__tests__/image-creation.spec.ts
git commit -m "feat: add image creation api contract"
```

### Task 2: Add image creation service logic

**Files:**
- Create: `src/services/image-creation.ts`
- Create: `src/services/__tests__/image-creation.spec.ts`

**Interfaces:**
- Consumes: `keysAPI.list()` from `src/api/keys.ts`, the image API functions from Task 1, and the existing `ApiKey` type.
- Produces: `loadImageCreationBootstrap()`, `selectDefaultImageCreationKey()`, `validateImageCreationDraft()`, `submitImageCreationDraft()`, `pollImageCreationJob()`, `hydrateDraftFromJob()`.

- [ ] **Step 1: Write the failing test**

```ts
it('selects the first active key and rejects more than four reference images', () => {
  const keys = [
    { id: 1, status: 'inactive' },
    { id: 2, status: 'active' },
    { id: 3, status: 'active' }
  ] as ApiKey[]

  expect(selectDefaultImageCreationKey(keys)?.id).toBe(2)
  expect(() =>
    validateImageCreationDraft({
      keyId: 2,
      prompt: 'sunset',
      negativePrompt: '',
      model: 'gpt-image-1',
      aspectRatio: '1:1',
      count: 1,
      referenceImages: makeFiveReferenceImages()
    })
  ).toThrow('referenceImages')
})
```

- [ ] **Step 2: Run test to verify it fails**

Run: `npx vitest run src/services/__tests__/image-creation.spec.ts`

Expected: FAIL because the selection and validation helpers do not exist yet.

- [ ] **Step 3: Write minimal implementation**

Keep the service thin: load active keys, pick a sane default key, enforce the 4-image cap, and submit jobs through the Task 1 API wrapper.

- [ ] **Step 4: Run test to verify it passes**

Run: `npx vitest run src/services/__tests__/image-creation.spec.ts`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/services/image-creation.ts src/services/__tests__/image-creation.spec.ts
git commit -m "feat: add image creation service"
```

### Task 3: Rebuild the image creation page

**Files:**
- Modify: `src/views/ImageCreationView.vue`
- Create: `src/components/image-creation/ImageCreationComposer.vue`
- Create: `src/components/image-creation/ImageCreationHistoryList.vue`
- Create: `src/components/image-creation/ImageCreationResultGrid.vue`
- Modify: `src/views/__tests__/image-creation-view.spec.ts`

**Interfaces:**
- Consumes: `loadImageCreationBootstrap()`, `submitImageCreationDraft()`, and the selected key from Task 2.
- Produces: a first-party workbench page with prompt input, reference image upload, model/ratio/count controls, result preview, download, retry, set-as-reference, and recent history.

- [ ] **Step 1: Write the failing test**

```ts
it('no longer embeds the Infinite Canvas iframe and exposes the native workbench sections', () => {
  const source = readFileSync(fileURLToPath(new URL('../ImageCreationView.vue', import.meta.url)), 'utf8')

  expect(source).not.toContain('http://127.0.0.1:3000/')
  expect(source).toContain('prompt')
  expect(source).toContain('referenceImages')
  expect(source).toContain('ImageCreationHistoryList')
})
```

- [ ] **Step 2: Run test to verify it fails**

Run: `npx vitest run src/views/__tests__/image-creation-view.spec.ts`

Expected: FAIL because the current page still embeds the old canvas flow.

- [ ] **Step 3: Write minimal implementation**

Replace the iframe shell with a real workbench layout. Keep the current route and sidebar label, but wire the page to the new composer, history, and result components.

- [ ] **Step 4: Run test to verify it passes**

Run: `npx vitest run src/views/__tests__/image-creation-view.spec.ts`

Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add src/views/ImageCreationView.vue src/components/image-creation/ImageCreationComposer.vue src/components/image-creation/ImageCreationHistoryList.vue src/components/image-creation/ImageCreationResultGrid.vue src/views/__tests__/image-creation-view.spec.ts
git commit -m "feat: rebuild image creation workbench"
```

### Task 4: Verify the full flow and clean up copy

**Files:**
- Modify: `src/i18n/locales/zh.ts`
- Modify: `src/i18n/locales/en.ts`
- Modify: `src/layout/UserLayout.vue` only if any image-creation copy needs to change
- Modify: `src/router/index.ts` only if the page wiring needs to be rechecked

**Interfaces:**
- Consumes: the finished workbench page and service from Tasks 1-3.
- Produces: final user-facing copy, regression coverage, and build verification.

- [ ] **Step 1: Update copy and regression tests**

Make sure no remaining user-facing text still references Infinite Canvas or the old embedded preview flow.

- [ ] **Step 2: Run the targeted tests**

Run:

```bash
npx vitest run src/api/__tests__/image-creation.spec.ts src/services/__tests__/image-creation.spec.ts src/views/__tests__/image-creation-view.spec.ts
```

Expected: PASS

- [ ] **Step 3: Run the production build**

Run:

```bash
npm run build
```

Expected: PASS

- [ ] **Step 4: Run desktop packaging on macOS**

Run:

```bash
npm run tauri:build -- --bundles dmg
```

Expected: PASS and produce a new `Hapi_*.dmg`.

- [ ] **Step 5: Commit**

```bash
git add src/i18n/locales/zh.ts src/i18n/locales/en.ts src/layout/UserLayout.vue src/router/index.ts
git commit -m "feat: finish image creation workbench"
```
