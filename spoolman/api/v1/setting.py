"""Setting related endpoints."""

import asyncio
import logging
from typing import Annotated, Union

from fastapi import APIRouter, Body, Depends
from fastapi.responses import JSONResponse

from spoolman.api.v1.models import Message, SettingResponse
from spoolman.settings import SETTINGS, parse_setting
from spoolman.storage.dependencies import get_store
from spoolman.storage.store import JsonStore
router = APIRouter(
    prefix="/setting",
    tags=["setting"],
)

# ruff: noqa: D103,B008

logger = logging.getLogger(__name__)


@router.get(
    "/{key}",
    name="Get setting",
    response_model_exclude_none=True,
    response_model=SettingResponse,
    responses={404: {"model": Message}},
)
async def get(
    store: Annotated[JsonStore, Depends(get_store)],
    key: str,
) -> Union[SettingResponse, JSONResponse]:
    try:
        definition = parse_setting(key)
    except ValueError as e:
        return JSONResponse(status_code=404, content=Message(message=str(e)).model_dump())

    value = store.get_setting(definition.key)
    is_set = value is not None
    if value is None:
        value = definition.default

    return SettingResponse(value=value, is_set=is_set, type=definition.type)


@router.get(
    "/",
    name="Get all settings",
    response_model_exclude_none=True,
    response_model=dict[str, SettingResponse],
)
async def find(
    store: Annotated[JsonStore, Depends(get_store)],
) -> dict[str, SettingResponse]:
    all_set = store.get_all_settings()
    settings: dict[str, SettingResponse] = {}

    for key, value in all_set.items():
        try:
            definition = parse_setting(key)
        except ValueError:
            continue
        settings[key] = SettingResponse(value=value, is_set=True, type=definition.type)

    for settingdef in SETTINGS.values():
        if settingdef.key not in settings:
            settings[settingdef.key] = SettingResponse(value=settingdef.default, is_set=False, type=settingdef.type)

    return settings


@router.post(
    "/{key}",
    name="Set setting",
    response_model_exclude_none=True,
    response_model=SettingResponse,
    responses={404: {"model": Message}},
)
async def update(
    store: Annotated[JsonStore, Depends(get_store)],
    key: str,
    body: Annotated[str, Body()],
) -> Union[SettingResponse, JSONResponse]:
    try:
        definition = parse_setting(key)
    except ValueError as e:
        return JSONResponse(status_code=404, content=Message(message=str(e)).model_dump())

    if body and body != "null":
        try:
            definition.validate_type(body)
        except ValueError as e:
            return JSONResponse(status_code=400, content=Message(message=str(e)).model_dump())

        await asyncio.to_thread(store.set_setting, definition.key, body)
        logger.info('Setting "%s" has been set to "%s".', key, body)
    else:
        await asyncio.to_thread(store.delete_setting, definition.key)
        logger.info('Setting "%s" has been unset.', key)

    value = store.get_setting(definition.key)
    is_set = value is not None
    if value is None:
        value = definition.default

    return SettingResponse(value=value, is_set=is_set, type=definition.type)
